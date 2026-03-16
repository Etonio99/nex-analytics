use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppData {
    pub subdomain: Option<String>,
    pub location_ids: Option<Vec<u32>>,
    pub excluded_location_ids: Option<Vec<u32>>,
}
