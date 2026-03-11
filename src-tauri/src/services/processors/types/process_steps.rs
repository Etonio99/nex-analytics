use serde::Serialize;

#[derive(Clone, PartialEq, Serialize)]
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
