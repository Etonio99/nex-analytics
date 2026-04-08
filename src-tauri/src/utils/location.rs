use crate::api::types::locations::Location;

fn non_empty(s: Option<&str>) -> Option<&str> {
    s.filter(|v| !v.is_empty())
}

pub fn format_location_address(location: &Location) -> String {
    let state_zip = match (
        non_empty(location.state.as_deref()),
        non_empty(location.zip_code.as_deref()),
    ) {
        (Some(state), Some(zip)) => Some(format!("{} {}", state, zip)),
        (Some(state), None) => Some(state.to_string()),
        (None, Some(zip)) => Some(zip.to_string()),
        (None, None) => None,
    };
    let address = [
        non_empty(location.street_address.as_deref()),
        non_empty(location.street_address_2.as_deref()),
        non_empty(location.city.as_deref()),
        non_empty(state_zip.as_deref()),
    ]
    .iter()
    .filter_map(|s| *s)
    .collect::<Vec<_>>()
    .join(", ");

    if address.is_empty() {
        "No address listed".to_string()
    } else {
        address
    }
}
