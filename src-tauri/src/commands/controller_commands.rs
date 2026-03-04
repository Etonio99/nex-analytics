use crate::{
    api::NexApiClient,
    services::{
        controller::Controller,
        processors::{
            appointment_slots_processor::AppointmentSlotsProcessor,
            types::processor_advance_result::ProcessorAdvanceResult,
        },
    },
};

#[tauri::command]
pub async fn set_processor(
    controller: tauri::State<'_, Controller>,
    processor_name: String,
) -> Result<(), String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

    match processor_name.as_str() {
        "appointment_slots" => {
            *lock = Some(Box::new(AppointmentSlotsProcessor::new()));
        }
        _ => return Err("Unknown processor name".into()),
    }

    Ok(())
}

// app.emit("processor-step", self.current_step.clone())?;

// #[tauri::command]
// pub async fn advance_processor(app: tauri::AppHandle, controller: tauri::State<'_, Controller>, client: tauri::State<'_, NexApiClient>) -> Result<(), String> {
//     let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

//     if let Some(ref mut processor) = *lock {
//         processor.advance(&client, &app).map_err(|e| e.to_string())?;
//         Ok(())
//     } else {
//         Err("No processor selected".into())
//     }
// }

#[tauri::command]
pub async fn advance_processor(
    app: tauri::AppHandle,
    controller: tauri::State<'_, Controller>,
    client: tauri::State<'_, NexApiClient>,
) -> Result<ProcessorAdvanceResult, String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

    let processor = lock.as_mut().ok_or("No processor selected")?;

    processor.advance(&client, &app)

    // if let Some(ref mut processor) = *lock {
    //     processor.advance(&client, &app).map_err(|e| e.to_string())?;
    //     Ok(())
    // } else {
    //     Err("No processor selected".into())
    // }
}

#[tauri::command]
pub async fn update_processor_data(
    controller: tauri::State<'_, Controller>,
    data: serde_json::Value,
) -> Result<(), String> {
    let mut lock = controller.processor.write().map_err(|_| "Lock poisoned")?;

    if let Some(ref mut processor) = *lock {
        processor.update_data(data)
    } else {
        Err("No processor active".into())
    }
}
