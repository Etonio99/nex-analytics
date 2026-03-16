use crate::{
    api::NexApiClient,
    services::{
        controller::Controller,
        processors::{
            appointment_slots_processor::AppointmentSlotsProcessor,
            types::{process_steps::ProcessStep, processor_advance_result::ProcessorAdvanceResult},
        },
    },
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
    let processor = guard.as_mut().ok_or("No processor selected")?;
    processor.update_data(data)
}

#[tauri::command]
pub async fn update_app_data(
    controller: tauri::State<'_, Controller>,
    data: serde_json::Value,
) -> Result<(), String> {
    controller.update_app_data(data).await?;
    Ok(())
}

#[tauri::command]
pub async fn make_stale(controller: tauri::State<'_, Controller>) -> Result<(), String> {
    let mut guard = controller.processor.lock().await;
    let processor = guard.as_mut().ok_or("No processor selected")?;
    processor.make_stale();

    Ok(())
}

#[tauri::command]
pub async fn jump_to_step(
    app: tauri::AppHandle,
    controller: tauri::State<'_, Controller>,
    client: tauri::State<'_, NexApiClient>,
    step: ProcessStep,
) -> Result<ProcessorAdvanceResult, String> {
    println!("Received {:?}", step);
    let mut guard = controller.processor.lock().await;
    let processor = guard.as_mut().ok_or("No processor selected")?;
    processor.jump_to_step(step);
    processor.advance(&client, &app).await
}
