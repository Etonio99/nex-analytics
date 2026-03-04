use chrono::NaiveDate;
use reqwest::Method;

use crate::{
    api::types::appointment_slots::AppointmentSlotsQuery, AppointmentSlotsResponse, NexApiClient,
    NexApiResponse,
};

impl NexApiClient {
    pub async fn get_appointment_slots(
        &self,
        query: AppointmentSlotsQuery,
    ) -> Result<NexApiResponse<Vec<AppointmentSlotsResponse>>, String> {
        let response = self
            .request::<Vec<AppointmentSlotsResponse>, (), AppointmentSlotsQuery>(
                "appointment_slots",
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
