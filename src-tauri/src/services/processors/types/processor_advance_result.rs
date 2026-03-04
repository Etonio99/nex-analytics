use serde::Serialize;

use crate::services::processors::types::process_steps::ProcessStep;

#[derive(Serialize)]
pub struct ProcessorAdvanceResult {
    pub step: ProcessStep,
    pub error: Option<String>,
}
