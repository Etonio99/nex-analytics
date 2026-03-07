use std::sync::Arc;

use crate::utils::app_state::AppState;

#[tauri::command]
pub async fn update_app_data(
    app_state: tauri::State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> Result<(), String> {
    app_state.update(data).await
}
