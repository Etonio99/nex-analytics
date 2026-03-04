use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentSlotsResponse {
    pub lid: u32,
    pub pid: Option<u32>,
    pub operatory_id: Option<u32>,
    pub slots: Option<Vec<AppointmentSlot>>,
    pub next_available_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentSlot {
    pub time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub operatory_id: Option<u32>,
    pub provider_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderLocationMap {
    pub location_id: u32,
    pub provider_ids: Vec<u32>,
}

#[derive(Serialize)]
pub struct AppointmentSlotsQuery {
    pub subdomain: String,
    #[serde(serialize_with = "serialize_date")]
    pub start_date: NaiveDate,
    pub days: u32,
    pub appointment_type_id: u32,

    #[serde(rename = "lids[]")]
    pub location_id: u32,

    #[serde(rename = "pids[]", serialize_with = "serialize_pids")]
    pub provider_ids: Vec<u32>,
}

pub fn serialize_pids<S>(ids: &[u32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join("pids[]=");
    serializer.serialize_str(&joined)
}

pub fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format("%Y-%m-%d"));
    serializer.serialize_str(&s)
}
