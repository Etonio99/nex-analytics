use async_trait::async_trait;

use crate::{
    api::NexApiClient,
    services::processors::types::{
        process_steps::ProcessStep, processor_advance_result::ProcessorAdvanceResult,
    },
};

#[async_trait]
pub trait Processor: Send + Sync {
    async fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String>;
    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String>;
    fn make_stale(&mut self);
    fn jump_to_step(&mut self, step: ProcessStep);
}
