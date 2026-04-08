use crate::{
    api::{
        types::{locations::InstitutionLocations, nex_api::NexApiResponse},
        NexApiClient,
    },
    services::controller::Controller,
};

#[tauri::command]
pub async fn get_locations(
    controller: tauri::State<'_, Controller>,
    client: tauri::State<'_, NexApiClient>,
    inactive: bool,
) -> Result<NexApiResponse<Vec<InstitutionLocations>>, String> {
    controller.get_locations(&client, inactive).await
}
