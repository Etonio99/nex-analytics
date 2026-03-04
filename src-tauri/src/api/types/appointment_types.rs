use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AppointmentTypesQuery {
    pub subdomain: String,
    pub location_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentType {
    pub id: u32,
    pub name: String,
    pub parent_type: String,
    pub parent_id: u32,
    pub minutes: u32,
    pub bookable_online: bool,
}
