use serde::{Deserialize, Serialize};

use super::utils::limit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hue {
    value: u16,
}

impl Hue {
    pub fn new(value: u16) -> Self {
        Self {
            value: limit(0, value, 359),
        }
    }

    pub fn get_value(&self) -> u16 {
        self.value
    }
}
