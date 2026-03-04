use std::sync::RwLock;

use crate::services::processors::traits::Processor;

pub struct Controller {
    pub subdomain: Option<String>,

    pub processor: RwLock<Option<Box<dyn Processor + Send + Sync>>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            subdomain: None,
            processor: RwLock::new(None),
        }
    }
}
