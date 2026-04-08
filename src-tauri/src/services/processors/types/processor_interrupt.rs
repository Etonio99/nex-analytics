use serde::Serialize;

use crate::{
    api::types::locations::Location,
    services::processors::types::data_confirmation::DataConfirmation,
};

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessorInterrupt {
    Error(ProcessorError),
    InputRequired(ProcessorInputRequest),
}

// pub enum ProcessorInterrupt {
//     MissingApiKey,
//     InvalidApiKey,
//     MissingSubdomain(Option<InterruptResolutionData>),
//     LocationRequired(Option<InterruptResolutionData>),
//     NoLocationsFound,
//     MissingStartDate(Option<InterruptResolutionData>),
//     MissingDays(Option<InterruptResolutionData>),
//     MissingAppointmentTypeName(Option<InterruptResolutionData>),
//     NeedsConfirmation(InterruptResolutionData),
//     AcknowledgeCompletion(InterruptResolutionData),
//     PermissionDenied(InterruptResolutionData),
//     NotFound,
//     InternalError(InterruptResolutionData),
// }

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessorError {
    InvalidApiKey,
    NoLocationsFound,
    PermissionDenied,
    InternalError,
}

#[derive(Serialize, Debug, Clone)]
pub struct ProcessorInputRequest {
    pub title: String,
    pub description: String,
    pub input_field: InputField,
}

#[derive(Serialize, Debug, Clone)]
pub struct InputField {
    pub data: InputData,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub description: Option<String>,
    pub key: String,
    pub required: bool,
}

// The optional value is a prefill or required data to create the input
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InputData {
    String(Option<String>),
    MultiString(MultiStringData),
    Date(Option<String>),
    Number(Option<u32>),
    Select(SelectData),
    Confirm(DataConfirmation),
    AcknowledgeCompletion,
}

#[derive(Serialize, Debug, Clone)]
pub struct MultiStringData {
    pub options: Vec<String>,
    pub selected_strings: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SelectData {
    pub options: Vec<SelectOption>,
    pub selected_keys: Option<Vec<u32>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SelectOption {
    pub title: String,
    pub subtitle: String,
    pub key: u32,
}
