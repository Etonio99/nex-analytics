use serde::{Deserialize, Serialize};

use crate::api::types::appointment_types::AppointmentType;

#[derive(Serialize)]
pub struct OperatoriesQuery {
    pub subdomain: String,
    pub location_id: u32,
    pub per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operatory {
    pub id: u32,
    pub name: String,
    pub foreign_id: Option<String>,
    pub foreign_id_type: Option<String>,
    pub location_id: u32,
    pub last_sync_time: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub display_name: Option<String>,
    pub profile_url: Option<String>,
    pub active: bool,
    pub bookable_online: bool,
    pub appt_categories: Vec<AppointmentType>,
}
