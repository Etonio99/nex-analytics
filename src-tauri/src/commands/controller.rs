use std::sync::Arc;

use crate::{
    api::NexApiClient,
    services::{
        controller::Controller,
        processors::{
            appointment_slots_processor::AppointmentSlotsProcessor,
            types::processor_advance_result::ProcessorAdvanceResult,
        },
    },
    utils::app_state::AppState,
};

#[tauri::command]
pub async fn set_processor(
    controller: tauri::State<'_, Controller>,
    processor_name: String,
) -> Result<(), String> {
    let mut guard = controller.processor.lock().await;

    match processor_name.as_str() {
        "appointment_slots" => {
            let shared_app_state = controller.app_state.clone();
            *guard = Some(Box::new(AppointmentSlotsProcessor::new(shared_app_state)));
        }
        _ => return Err("Unknown processor name".into()),
    }

    Ok(())
}

#[tauri::command]
pub async fn advance_processor(
    app: tauri::AppHandle,
    controller: tauri::State<'_, Controller>,
    client: tauri::State<'_, NexApiClient>,
) -> Result<ProcessorAdvanceResult, String> {
    let mut guard = controller.processor.lock().await;

    let processor = guard.as_mut().ok_or("No processor selected")?;

    processor.advance(&client, &app).await
}

#[tauri::command]
pub async fn update_processor_data(
    controller: tauri::State<'_, Controller>,
    data: serde_json::Value,
) -> Result<(), String> {
    let mut guard = controller.processor.lock().await;

    if let Some(ref mut processor) = *guard {
        processor.update_data(data)
    } else {
        Err("No processor active".into())
    }
}
