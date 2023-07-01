use serde::{Deserialize, Serialize};

use super::utils::limit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Saturation {
    value: u8,
}

impl Saturation {
    pub fn new(value: u8) -> Self {
        Self {
            value: limit(0, value, 100),
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}
