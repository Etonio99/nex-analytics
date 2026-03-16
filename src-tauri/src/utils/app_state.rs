use std::{fs, path::PathBuf};

use tokio::sync::Mutex;

use crate::utils::AppData;

pub struct AppState {
    pub data: Mutex<AppData>,
    pub path: PathBuf,
}

impl AppState {
    pub fn new(path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let data = if path.exists() {
            let contents = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            AppData::default()
        };

        Self {
            data: Mutex::new(data),
            path,
        }
    }

    pub async fn update(
        &self,
        data: serde_json::Value,
    ) -> Result<Vec<AppDataUpdateResponse>, String> {
        let mut responses: Vec<AppDataUpdateResponse> = Vec::new();

        let mut guard = self.data.lock().await;

        let mut current_value = serde_json::to_value(&*guard).map_err(|e| e.to_string())?;

        if let Some(current_data) = current_value.as_object_mut() {
            if let Some(update_data) = data.as_object() {
                for (key, value) in update_data {
                    if !value.is_null() {
                        if key == "subdomain" {
                            let new_subdomain = value.as_str().map(|s| s.to_string());

                            if new_subdomain != guard.subdomain {
                                responses.push(AppDataUpdateResponse::MakeProcessorStale);
                            }
                        }

                        current_data.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        let updated_data: AppData = serde_json::from_value(current_value)
            .map_err(|e| format!("Resulting data is invalid: {}", e))?;

        *guard = updated_data;
        let json_string = serde_json::to_string_pretty(&*guard).map_err(|e| e.to_string())?;
        fs::write(&self.path, json_string).map_err(|e| e.to_string())?;

        Ok(responses)
    }
}

pub enum AppDataUpdateResponse {
    MakeProcessorStale,
}
