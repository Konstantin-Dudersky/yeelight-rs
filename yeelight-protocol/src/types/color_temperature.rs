use serde::{Deserialize, Serialize};

use super::utils::limit;

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorTemperature {
    value: u32,
}

impl ColorTemperature {
    pub fn new(value: u32) -> Self {
        Self {
            value: limit(1700, value, 6500),
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
