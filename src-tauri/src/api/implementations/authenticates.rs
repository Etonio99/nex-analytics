use reqwest::Method;

use crate::{api::types::authenticates::Authentication, NexApiClient, NexApiResponse};

impl NexApiClient {
    pub async fn get_authenticates(&self) -> Result<NexApiResponse<Authentication>, String> {
        let response = self
            .request::<Authentication, (), ()>("authenticates", Method::POST, None, None, false)
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }
}
