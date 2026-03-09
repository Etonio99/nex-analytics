use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    api::{
        types::{locations::LocationsQuery, operatories::Operatory, providers::Provider},
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{
            process_steps::ProcessStep,
            processor_advance_result::ProcessorAdvanceResult,
            processor_error::{ErrorResolutionData, ProcessorError},
        },
    },
    utils::app_state::{self, AppState},
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub data: AppointmentSlotsProcessorData,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlotsProcessorData {
    pub location_ids: Option<Vec<u32>>,
    pub days: Option<u32>,
    pub appointment_type_id: Option<u32>,
    pub operatories: Option<Vec<Operatory>>,
    pub providers: Option<Vec<Provider>>,
}

impl AppointmentSlotsProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            current_step: ProcessStep::CheckApiKey,
            data: AppointmentSlotsProcessorData {
                location_ids: None,
                days: None,
                appointment_type_id: None,
                operatories: None,
                providers: None,
            },
        }
    }

    async fn step(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<bool, ProcessorError> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()
                    .map_err(|e| {
                        ProcessorError::InternalError(ErrorResolutionData::Message(e.to_string()))
                    })?
                    .is_none()
                {
                    return Err(ProcessorError::MissingApiKey);
                }

                let response = client.get_authenticates().await.map_err(|e| {
                    ProcessorError::InternalError(ErrorResolutionData::Message(e.to_string()))
                })?;
                if !response.code {
                    return Err(ProcessorError::InvalidApiKey);
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                let _ = guard
                    .subdomain
                    .as_ref()
                    .ok_or(ProcessorError::MissingSubdomain)?;

                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.location_ids.as_ref().filter(|l| !l.is_empty()) else {
                    let guard = self.app_state.data.lock().await;

                    let subdomain = guard
                        .subdomain
                        .as_ref()
                        .ok_or(ProcessorError::MissingSubdomain)?;

                    let locations_response = client
                        .get_locations(LocationsQuery {
                            subdomain: subdomain.clone(),
                            inactive: false,
                        })
                        .await;

                    return match locations_response {
                        Ok(res) => {
                            if let Some(loc_wrapper) = res.data {
                                Err(ProcessorError::LocationRequired(
                                    ErrorResolutionData::Locations(
                                        loc_wrapper[0].locations.clone(),
                                    ),
                                ))
                            } else {
                                Err(ProcessorError::LocationRequired(ErrorResolutionData::None))
                            }
                        }
                        Err(e) => Err(ProcessorError::InternalError(ErrorResolutionData::Message(
                            e.to_string(),
                        ))),
                    };
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(ProcessorError::MissingDays);
                };
                self.current_step = ProcessStep::SelectLocations;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}

#[async_trait]
impl Processor for AppointmentSlotsProcessor {
    async fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String> {
        let mut error = None;

        loop {
            match self.step(client, app).await {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }

        Ok(ProcessorAdvanceResult {
            step: self.current_step.clone(),
            error,
        })
    }

    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String> {
        let input: AppointmentSlotsProcessorData = serde_json::from_value(data)
            .map_err(|e| format!("Invalid data for Appointment Slots Processor: {}", e))?;

        if let Some(l) = input.location_ids {
            self.data.location_ids = Some(l);
        }
        if let Some(d) = input.days {
            self.data.days = Some(d);
        }
        if let Some(a) = input.appointment_type_id {
            self.data.appointment_type_id = Some(a);
        }
        if let Some(o) = input.operatories {
            self.data.operatories = Some(o);
        }
        if let Some(p) = input.providers {
            self.data.providers = Some(p);
        }

        Ok(())
    }
}
