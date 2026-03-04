use serde::Deserialize;

use crate::{
    api::{
        types::{operatories::Operatory, providers::Provider},
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{process_steps::ProcessStep, processor_advance_result::ProcessorAdvanceResult},
    },
};

pub struct AppointmentSlotsProcessor {
    pub current_step: ProcessStep,
    pub data: AppointmentSlotsProcessorData,
}

#[derive(Deserialize)]
pub struct AppointmentSlotsProcessorData {
    pub subdomain: Option<String>,
    pub locations: Option<Vec<u32>>,
    pub days: Option<u32>,
    pub appointment_type_id: Option<u32>,
    pub operatories: Option<Vec<Operatory>>,
    pub providers: Option<Vec<Provider>>,
}

impl AppointmentSlotsProcessor {
    pub fn new() -> Self {
        Self {
            current_step: ProcessStep::CheckApiKey,
            data: AppointmentSlotsProcessorData {
                subdomain: None,
                locations: None,
                days: None,
                appointment_type_id: None,
                operatories: None,
                providers: None,
            },
        }
    }

    fn step(&mut self, client: &NexApiClient, app: &tauri::AppHandle) -> Result<bool, String> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()?.is_none() {
                    return Err("Api key is required".into());
                }
                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let Some(_) = self.data.subdomain else {
                    return Err("Subdomain is required".into());
                };
                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.locations.as_ref().filter(|l| !l.is_empty()) else {
                    return Err("At least one location must be selected".into());
                };
                self.current_step = ProcessStep::EnterDays;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}

impl Processor for AppointmentSlotsProcessor {
    fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String> {
        let mut error = None;

        loop {
            match self.step(client, app) {
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

        if let Some(s) = input.subdomain {
            self.data.subdomain = Some(s);
        }
        if let Some(l) = input.locations {
            self.data.locations = Some(l);
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
