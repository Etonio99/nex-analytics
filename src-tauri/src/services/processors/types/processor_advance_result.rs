use serde::Serialize;

use crate::services::processors::types::{process_steps::ProcessStep, processor_error::ProcessorError};

#[derive(Serialize)]
pub struct ProcessorAdvanceResult {
    pub step: ProcessStep,
    pub error: Option<ProcessorError>,
}
