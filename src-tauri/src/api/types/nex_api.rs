use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NexApiResponse<T> {
    pub code: bool,
    pub data: Option<T>,
    pub description: Option<Vec<String>>,
    pub error: Option<Vec<String>>,
    pub count: Option<u64>,
}
