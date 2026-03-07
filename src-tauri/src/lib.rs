mod api;
mod commands;
mod services;
mod utils;
use std::sync::Arc;

use tauri::Manager;

use crate::{
    api::{
        types::{
            appointment_slots::AppointmentSlotsResponse, locations::LocationsQuery,
            nex_api::NexApiResponse,
        },
        NexApiClient,
    },
    commands::{
        api_calls::get_locations,
        app_state::update_app_data,
        controller::{advance_processor, set_processor, update_processor_data},
        keys::{get_api_key, save_api_key},
    },
    services::controller::Controller,
    utils::app_state::AppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = NexApiClient::new();

            let path = app.path().app_config_dir()?.join("config.json");
            println!("Config path: {:?}", path);
            let app_state = Arc::new(AppState::new(path));

            app.manage(client);
            app.manage(app_state.clone());
            app.manage(Controller::new(app_state));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_api_key,
            get_api_key,
            set_processor,
            advance_processor,
            update_processor_data,
            update_app_data,
            get_locations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
