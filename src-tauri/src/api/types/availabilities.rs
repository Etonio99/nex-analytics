use serde::{Deserialize, Serialize};

use crate::api::types::appointment_types::AppointmentType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    id: u32,
    provider_id: Option<u32>,
    location_id: Option<u32>,
    operatory_id: Option<u32>,
    begin_time: String,
    end_time: String,
    days: Vec<String>,
    specific_date: Option<String>,
    custom_recurrence: Option<CustomRecurrence>,
    tz_offset: String,
    active: bool,
    synced: bool,
    appointment_types: Vec<AppointmentType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomRecurrence {
    num: u32,
    unit: String,
    #[serde(rename = "ref")]
    reference: String,
}
