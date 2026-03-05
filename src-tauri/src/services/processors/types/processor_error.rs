use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "details", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessorError {
    MissingApiKey,
    InvalidApiKey,
    MissingSubdomain,
    LocationRequired,
    InternalError(String),
}