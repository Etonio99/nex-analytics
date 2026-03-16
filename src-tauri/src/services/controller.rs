use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    api::{
        types::{
            locations::{InstitutionLocations, LocationsQuery},
            nex_api::NexApiResponse,
        },
        NexApiClient,
    },
    services::processors::traits::Processor,
    utils::app_state::{AppDataUpdateResponse, AppState},
};

pub struct Controller {
    pub app_state: Arc<AppState>,
    pub processor: Mutex<Option<Box<dyn Processor + Send + Sync>>>,
}

impl Controller {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            processor: Mutex::new(None),
        }
    }

    pub async fn get_locations(
        &self,
        client: &NexApiClient,
        inactive: bool,
    ) -> Result<NexApiResponse<Vec<InstitutionLocations>>, String> {
        let guard = self.app_state.data.lock().await;

        let subdomain = guard
            .subdomain
            .as_ref()
            .ok_or("No subdomain in app state")?;

        let response = client
            .get_locations(LocationsQuery {
                subdomain: subdomain.clone(),
                inactive,
            })
            .await
            .map_err(|e| e.to_string())?;

        Ok(response)
    }

    pub async fn update_app_data(&self, data: serde_json::Value) -> Result<(), String> {
        let responses = self.app_state.update(data).await?;

        for response in responses {
            match response {
                AppDataUpdateResponse::MakeProcessorStale => {
                    let mut guard = self.processor.lock().await;

                    if let Some(processor) = guard.as_mut() {
                        processor.make_stale();
                    }
                }
            }
        }

        Ok(())
    }
}
