use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bio {
    city: Option<String>,
    state: Option<String>,
    gender: Option<String>,
    zip_code: Option<String>,
    new_patient: Option<bool>,
    non_patient: Option<bool>,
    phone_number: Option<String>,
    date_of_birth: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    street_address: Option<String>,
    cell_phone_number: Option<String>,
    home_phone_number: Option<String>,
    work_phone_number: Option<String>,
}
