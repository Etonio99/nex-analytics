use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ProcessStep {
    CheckApiKey,
    EnterSubdomain,
    SelectLocations,
    SelectAppointmentType,
    EnterDays,
    CollectContext,
    CollectAnalytics,
    Complete,
}