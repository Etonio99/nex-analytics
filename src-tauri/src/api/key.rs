use keyring::{Entry, Error};

#[tauri::command]
pub fn save_api_key(key: String) -> Result<(), String> {
    let entry = Entry::new("nex-analytics", "api_key").map_err(|e| e.to_string())?;
    entry.set_password(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key() -> Result<Option<String>, String> {
    let entry = Entry::new("nex-analytics", "api_key").map_err(|e| e.to_string())?;
    
    match entry.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}