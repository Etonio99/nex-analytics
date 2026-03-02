use crate::api::NexApiClient;

pub trait Processor {
    fn advance(&mut self, client: &NexApiClient, app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>>;
    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String>;
}