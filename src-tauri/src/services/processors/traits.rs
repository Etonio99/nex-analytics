use crate::{
    api::NexApiClient,
    services::processors::types::processor_advance_result::ProcessorAdvanceResult,
};

pub trait Processor {
    fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String>;
    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String>;
}
