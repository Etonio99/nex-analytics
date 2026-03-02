use tauri::Runtime;

use crate::{api::NexApiClient, services::{controller::Controller, processors::{appointment_slots_processor::AppointmentSlotsProcessor, traits::Processor}}};

#[tauri::command]
async fn set_processor(processor_name: String, controller: tauri::State<'_, Controller>) -> Result<(), String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

    match processor_name.as_str() {
        "appointment_slots" => {
            *lock = Some(Box::new(AppointmentSlotsProcessor::new()));
        }
        _ => return Err("Unknown processor name".into()),
    }

    Ok(())
}

#[tauri::command]
async fn advance_processor(app: tauri::AppHandle, controller: tauri::State<'_, Controller>, client: tauri::State<'_, NexApiClient>) -> Result<(), String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;
    
    if let Some(ref mut processor) = *lock {
        let result = processor.advance(&client, &app); 
        Ok(())
    } else {
        Err("No processor selected".into())
    }
}

async fn update_processor_data(controller: tauri::State<'_, Controller>, data: serde_json::Value) -> Result<(), String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

    if let Some(ref mut processor) = *lock {
        processor.update_data(data)
    } else {
        Err("No processor active".into())
    }
}