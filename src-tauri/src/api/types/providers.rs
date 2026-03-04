use serde::{Deserialize, Serialize};

use crate::api::types::{availabilities::Availability, bio::Bio, locations::Location};

#[derive(Serialize)]
pub struct ProvidersQuery {
    pub subdomain: String,
    pub location_id: u32,
    pub inactive: bool,
    pub per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: u32,
    pub email: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub institution_id: u32,
    pub foreign_id: Option<String>,
    pub foreign_id_type: Option<String>,
    pub bio: Bio,
    pub inactive: bool,
    pub last_sync_time: Option<String>,
    pub display_name: Option<String>,
    pub npi: Option<String>,
    pub tin: Option<String>,
    pub state_license: Option<String>,
    pub specialty_code: Option<String>,
    pub nexhealth_specialty: Option<String>,
    pub locations: Vec<Location>,
    pub provider_requestables: Vec<ProviderRequestable>,
    pub availabilities: Option<Vec<Availability>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderRequestable {
    pub location_id: u32,
}
