use serde::Deserialize;
use tauri::Emitter;

use crate::{api::{NexApiClient, key::get_api_key, types::{operatories::Operatory, providers::Provider}}, services::processors::{traits::Processor, types::process_steps::ProcessStep}};

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
            }
        }
    }
}

impl Processor for AppointmentSlotsProcessor {
    fn advance(&mut self, client: &NexApiClient, app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()?.is_none() {
                    return Err("API key is required".into());
                }
            },
            ProcessStep::EnterSubdomain => {
                let Some(_) = self.data.subdomain else {
                    return Err("Subdomain is required".into());
                };
                self.current_step = ProcessStep::SelectLocations;
            },
            _ => {},
        }

        app.emit("processor-step", self.current_step.clone())?;

        Ok(())
    }

    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String> {
        let input: AppointmentSlotsProcessorData = serde_json::from_value(data)
            .map_err(|e| format!("Invalid data for Appointment Slots Processor: {}", e))?;

        if let Some(s) = input.subdomain { self.data.subdomain = Some(s); }
        if let Some(l) = input.locations { self.data.locations = Some(l); }
        if let Some(d) = input.days { self.data.days = Some(d); }
        if let Some(a) = input.appointment_type_id { self.data.appointment_type_id = Some(a); }
        if let Some(o) = input.operatories { self.data.operatories = Some(o); }
        if let Some(p) = input.providers { self.data.providers = Some(p); }
        
        Ok(())
    }
}