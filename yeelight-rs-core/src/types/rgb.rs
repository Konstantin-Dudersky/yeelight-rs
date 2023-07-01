use serde::{Deserialize, Serialize};

use super::utils::limit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rgb {
    value: u32,
}

impl Rgb {
    pub fn new(value: u32) -> Self {
        Self {
            value: limit(0, value, 16777215),
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
