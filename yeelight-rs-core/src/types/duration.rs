use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Duration {
    value: u32,
}

impl Duration {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
