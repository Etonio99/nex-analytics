use serde::Serialize;

use crate::{
    api::types::locations::Location,
    services::processors::types::data_confirmation::DataConfirmation,
};

#[derive(Serialize, Debug, Clone)]
#[serde(
    tag = "type",
    content = "resolutionData",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ProcessorInterrupt {
    MissingApiKey,
    InvalidApiKey,
    MissingSubdomain(Option<InterruptResolutionData>),
    LocationRequired(Option<InterruptResolutionData>),
    NoLocationsFound,
    MissingDays(Option<InterruptResolutionData>),
    MissingAppointmentTypeName(Option<InterruptResolutionData>),
    NeedsConfirmation(InterruptResolutionData),
    PermissionDenied(InterruptResolutionData),
    NotFound,
    InternalError(InterruptResolutionData),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterruptResolutionData {
    String(String),
    Number(u32),
    Locations(LocationResolutionData),
    Confirmation(DataConfirmation),
    None,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocationResolutionData {
    pub locations: Vec<Location>,
    pub selected_location_ids: Option<Vec<u32>>,
}
