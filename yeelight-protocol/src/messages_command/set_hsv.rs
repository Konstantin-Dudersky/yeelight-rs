use serde::{Deserialize, Serialize};

use super::super::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetHsv {
    id: i32,
    method: String,
    params: (u16, u8, types::Effect, u32),
}

impl SetHsv {
    pub fn new(
        hue: types::Hue,
        sat: types::Brightness,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Self {
        Self {
            id: 1,
            method: String::from("set_hsv"),
            params: (
                hue.get_value(),
                sat.get_value(),
                effect,
                duration.get_value(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn new() {
        let msg = SetHsv::new(
            types::Hue::new(255),
            types::Brightness::new(45),
            types::Effect::Smooth,
            types::Duration::new(500),
        );
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json =
            r#"{"id":1,"method":"set_hsv","params":[255,45,"smooth",500]}"#;
        assert_eq!(msg_serial, json);
    }
}
