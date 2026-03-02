mod api;
mod commands;
mod services;
mod utils;
use std::sync::Mutex;

use crate::{
    api::{
        NexApiClient, key::{get_api_key, save_api_key}, types::{
            appointment_slots::AppointmentSlotsResponse,locations::LocationsQuery,
            nex_api::NexApiResponse,
        }
    }, commands::controller_commands::{advance_processor, set_processor, update_processor_data}, services::controller::Controller, utils::AppData
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = NexApiClient::new(Some("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM1Iiwic2NwIjoiYXBpX3VzZXIiLCJpYXQiOjE3NzIyNTI0ODUsImV4cCI6MTc3MjI1NjA4NSwianRpIjoiM2IzMGUwM2YtMDhhNy00YmRjLWExYTAtMTQ0ZGE0ODIwYjE3In0.9-IAk1LNioY5CSuvrvB0c4VdNkE6egnPP_qeiYKNSkY".into()));
    let app_data = Mutex::new(AppData {
        location_ids: vec![],
        excluded_location_ids: vec![],
    });

    let controller = Controller::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(client)
        .manage(app_data)
        .manage(controller)
        .invoke_handler(tauri::generate_handler![
            save_api_key,
            get_api_key,
            set_processor,
            advance_processor,
            update_processor_data,    
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
