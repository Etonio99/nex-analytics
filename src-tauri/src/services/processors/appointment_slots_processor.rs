use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    api::{
        types::{
            locations::{Location, LocationsQuery},
            operatories::Operatory,
            providers::Provider,
        },
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{
            data_confirmation::DataConfirmation,
            process_steps::ProcessStep,
            processor_advance_result::ProcessorAdvanceResult,
            processor_interrupt::{
                InterruptResolutionData, LocationResolutionData, ProcessorInterrupt,
            },
        },
    },
    utils::app_state::AppState,
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub target_step: Option<ProcessStep>,
    pub data: AppointmentSlotsProcessorData,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlotsProcessorData {
    pub confirmed: Option<bool>,
    pub locations: Option<Vec<Location>>,
    pub selected_location_ids: Option<Vec<u32>>,
    pub days: Option<u32>,
    pub appointment_type_name: Option<String>,
    pub operatories: Option<Vec<Operatory>>,
    pub providers: Option<Vec<Provider>>,
}

impl AppointmentSlotsProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            current_step: ProcessStep::CheckApiKey,
            target_step: None,
            data: AppointmentSlotsProcessorData {
                confirmed: Some(false),
                locations: None,
                selected_location_ids: None,
                days: None,
                appointment_type_name: None,
                operatories: None,
                providers: None,
            },
        }
    }

    async fn step(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<bool, ProcessorInterrupt> {
        if Some(self.current_step.clone()) == self.target_step {
            self.target_step = None;
            return Err(self.get_interrupt_for_current_step().await);
        }

        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?
                    .is_none()
                {
                    return Err(ProcessorInterrupt::MissingApiKey);
                }

                let response = client.get_authenticates().await.map_err(|e| {
                    ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                        e.to_string(),
                    ))
                })?;
                if !response.code {
                    return Err(ProcessorInterrupt::InvalidApiKey);
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return Err(ProcessorInterrupt::MissingSubdomain(None));
                }

                self.current_step = ProcessStep::FetchLocations;
            }
            ProcessStep::FetchLocations => {
                if self.data.locations.is_none() {
                    let guard = self.app_state.data.lock().await;

                    let Some(subdomain) = guard.subdomain.as_ref() else {
                        return Err(ProcessorInterrupt::MissingSubdomain(None));
                    };

                    let locations_response = client
                        .get_locations(LocationsQuery {
                            subdomain: subdomain.clone(),
                            inactive: false,
                        })
                        .await
                        .map_err(|e| {
                            ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                                e.to_string(),
                            ))
                        })?;

                    println!("{:#?}", locations_response);

                    if !locations_response.code {
                        if let Some(e) = locations_response.error {
                            if e.contains(
                                &"You don't have access to perform this action.".to_string(),
                            ) {
                                let permission_type = &Some("subdomain".to_string());
                                return Err(ProcessorInterrupt::PermissionDenied(
                                    self.wrap_str(permission_type)
                                        .unwrap_or(InterruptResolutionData::None),
                                ));
                            }
                        }
                    }

                    if let Some(institution_locations) = locations_response.data {
                        self.data.locations = Some(institution_locations[0].locations.clone());
                    } else {
                        return Err(ProcessorInterrupt::NoLocationsFound);
                    }
                }

                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.selected_location_ids else {
                    return Err(ProcessorInterrupt::LocationRequired(Some(
                        InterruptResolutionData::Locations(LocationResolutionData {
                            locations: self.data.locations.clone().unwrap_or_default(),
                            selected_location_ids: None,
                        }),
                    )));
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(ProcessorInterrupt::MissingDays(None));
                };
                self.current_step = ProcessStep::EnterAppointmentTypeName;
            }
            ProcessStep::EnterAppointmentTypeName => {
                let Some(_) = self.data.appointment_type_name else {
                    return Err(ProcessorInterrupt::MissingAppointmentTypeName(None));
                };
                self.current_step = ProcessStep::Confirmation;
            }
            ProcessStep::Confirmation => {
                if !self.data.confirmed.unwrap_or(false) {
                    let guard = self.app_state.data.lock().await;

                    let locations_count = self
                        .data
                        .selected_location_ids
                        .as_ref()
                        .map(|v| v.len() as u32);

                    return Err(ProcessorInterrupt::NeedsConfirmation(
                        InterruptResolutionData::Confirmation(DataConfirmation {
                            subdomain: guard.subdomain.clone(),
                            locations_count,
                            days: self.data.days,
                            appointment_type_name: self.data.appointment_type_name.clone(),
                        }),
                    ));
                }
                self.current_step = ProcessStep::Processing;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    async fn get_interrupt_for_current_step(&self) -> ProcessorInterrupt {
        match self.current_step {
            ProcessStep::CheckApiKey => ProcessorInterrupt::MissingApiKey,
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return ProcessorInterrupt::MissingSubdomain(None);
                }

                ProcessorInterrupt::MissingSubdomain(self.wrap_str(&guard.subdomain))
            }
            ProcessStep::SelectLocations => ProcessorInterrupt::LocationRequired(Some(
                InterruptResolutionData::Locations(LocationResolutionData {
                    locations: self.data.locations.clone().unwrap_or_default(),
                    selected_location_ids: self.data.selected_location_ids.clone(),
                }),
            )),
            ProcessStep::EnterDays => {
                ProcessorInterrupt::MissingDays(self.wrap_num(self.data.days))
            }
            ProcessStep::EnterAppointmentTypeName => {
                ProcessorInterrupt::MissingAppointmentTypeName(
                    self.wrap_str(&self.data.appointment_type_name),
                )
            }
            _ => ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                "Unknown step".into(),
            )),
        }
    }

    fn wrap_str(&self, value: &Option<String>) -> Option<InterruptResolutionData> {
        value
            .as_ref()
            .map(|s| InterruptResolutionData::String(s.clone()))
    }

    fn wrap_num(&self, value: Option<u32>) -> Option<InterruptResolutionData> {
        value.map(InterruptResolutionData::Number)
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
            println!("Now advancing to {:?}", self.current_step);
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

        if let Some(c) = input.confirmed {
            self.data.confirmed = Some(c);
        }
        if let Some(l) = input.locations {
            self.data.locations = Some(l);
        }
        if let Some(l) = input.selected_location_ids {
            self.data.selected_location_ids = Some(l);
        }
        if let Some(d) = input.days {
            self.data.days = Some(d);
        }
        if let Some(a) = input.appointment_type_name {
            self.data.appointment_type_name = Some(a);
        }
        if let Some(o) = input.operatories {
            self.data.operatories = Some(o);
        }
        if let Some(p) = input.providers {
            self.data.providers = Some(p);
        }

        Ok(())
    }

    fn make_stale(&mut self) {
        self.data.locations = None;
        self.data.selected_location_ids = None;
    }

    fn jump_to_step(&mut self, step: ProcessStep) {
        self.current_step = step.clone();
        self.target_step = Some(step.clone());
        println!("Jumped to step {:?}", step);
    }
}
