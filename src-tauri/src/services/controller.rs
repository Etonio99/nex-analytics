use std::sync::RwLock;

use crate::services::processors::traits::Processor;

pub struct Controller {
    pub processor: RwLock<Option<Box<dyn Processor + Send + Sync>>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            processor: RwLock::new(None),
        }
    }
}
