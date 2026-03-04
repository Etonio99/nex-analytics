use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppData {
    pub location_ids: Vec<u32>,
    pub excluded_location_ids: Vec<u32>,
}

impl AppData {
    pub fn load<R: Runtime>(app: &AppHandle<R>) -> Self {
        let path = app.path().app_config_dir().unwrap().join("config.json");
        fs::read_to_string(path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save<R: Runtime>(&self, app: &AppHandle<R>) {
        let path = app.path().app_config_dir().unwrap().join("config.json");
        let _ = fs::create_dir_all(path.parent().unwrap());
        let _ = fs::write(path, serde_json::to_string_pretty(self).unwrap());
    }
}
