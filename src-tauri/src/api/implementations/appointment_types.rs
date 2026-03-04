use reqwest::Method;

use crate::{
    api::types::appointment_types::{AppointmentType, AppointmentTypesQuery},
    NexApiClient, NexApiResponse,
};

impl NexApiClient {
    pub async fn get_appointment_types(
        &self,
        query: AppointmentTypesQuery,
    ) -> Result<NexApiResponse<Vec<AppointmentType>>, String> {
        let response = self
            .request::<Vec<AppointmentType>, (), AppointmentTypesQuery>(
                "appointment_types",
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
