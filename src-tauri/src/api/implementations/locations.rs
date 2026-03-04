use reqwest::Method;

use crate::{
    api::types::locations::InstitutionLocations, LocationsQuery, NexApiClient, NexApiResponse,
};

impl NexApiClient {
    pub async fn get_locations(
        &self,
        query: LocationsQuery,
    ) -> Result<NexApiResponse<Vec<InstitutionLocations>>, String> {
        let response = self
            .request::<Vec<InstitutionLocations>, (), LocationsQuery>(
                "locations",
                Method::GET,
                None,
                Some(&query),
                false,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}
