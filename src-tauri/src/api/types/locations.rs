use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionLocations {
    pub id: u32,
    pub name: String,
    pub subdomain: String,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub institution_id: u32,
    pub street_address: Option<String>,
    pub street_address_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone_number: Option<String>,
    pub foreign_id: Option<String>,
    pub foreign_id_type: Option<String>,
    pub email: Option<String>,
    pub tz: String,
    pub last_sync_time: Option<String>,
    pub insert_appt_client: bool,
    pub map_by_operatory: bool,
    pub appt_types_map_by_operatory: bool,
    pub set_availability_by_operatory: bool,
    pub inactive: bool,
    pub wlogo: Option<String>,
    pub weight: Option<u32>,
}

#[derive(Serialize)]
pub struct LocationsQuery {
    pub subdomain: String,
    pub inactive: bool,
}
