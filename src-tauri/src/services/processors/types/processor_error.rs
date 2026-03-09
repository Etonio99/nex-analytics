use serde::Serialize;

use crate::api::types::locations::Location;

#[derive(Serialize, Debug, Clone)]
#[serde(
    tag = "type",
    content = "resolutionData",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ProcessorError {
    MissingApiKey,
    InvalidApiKey,
    MissingSubdomain,
    LocationRequired(ErrorResolutionData),
    MissingDays,
    InternalError(ErrorResolutionData),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ErrorResolutionData {
    Message(String),
    Locations(Vec<Location>),
    None,
}
