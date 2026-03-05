use tokio::sync::Mutex;

use crate::services::processors::traits::Processor;

pub struct Controller {
    pub subdomain: Option<String>,

    pub processor: Mutex<Option<Box<dyn Processor + Send + Sync>>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            subdomain: None,
            processor: Mutex::new(None),
        }
    }
}
