use serde::{Deserialize, Serialize};

use super::super::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetRgb {
    id: i32,
    method: String,
    params: (u32, types::Effect, u32),
}

impl SetRgb {
    pub fn new(
        rgb_value: types::Rgb,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Self {
        Self {
            id: 1,
            method: String::from("set_rgb"),
            params: (rgb_value.get_value(), effect, duration.get_value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn set_rgb() {
        let msg = SetRgb::new(
            types::Rgb::new(255),
            types::Effect::Smooth,
            types::Duration::new(500),
        );
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json = r#"{"id":1,"method":"set_rgb","params":[255,"smooth",500]}"#;
        assert_eq!(msg_serial, json);
    }
}
