use serde::Serialize;

use crate::services::processors::types::{
    process_steps::ProcessStep, processor_interrupt::ProcessorInterrupt,
};

#[derive(Serialize)]
pub struct ProcessorAdvanceResult {
    pub step: ProcessStep,
    pub interrupt: Option<ProcessorInterrupt>,
}
