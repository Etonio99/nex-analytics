use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager, Runtime};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppData {
    pub subdomain: Option<String>,
    pub location_ids: Option<Vec<u32>>,
    pub excluded_location_ids: Option<Vec<u32>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            subdomain: None,
            location_ids: None,
            excluded_location_ids: None,
        }
    }
}
