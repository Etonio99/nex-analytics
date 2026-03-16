use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    FetchLocations,
    SelectLocations,
    EnterAppointmentTypeName,
    EnterDays,
    Confirmation,
    Processing,
    Complete,
}
