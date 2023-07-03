use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Duration {
    duration: u32,
}

impl Duration {
    pub fn new(value: u32) -> Self {
        Self { duration: value }
    }

    pub fn get_value(&self) -> u32 {
        self.duration
    }
}
