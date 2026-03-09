use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    FetchLocations,
    SelectLocations,
    SelectAppointmentType,
    EnterDays,
    CollectContext,
    CollectAnalytics,
    Complete,
}
