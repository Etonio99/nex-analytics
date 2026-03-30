use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DataConfirmation {
    pub subdomain: Option<String>,
    pub locations_count: Option<u32>,
    pub start_date: Option<String>,
    pub days: Option<u32>,
    pub appointment_type_name: Option<String>,
}
