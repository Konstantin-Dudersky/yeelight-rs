use serde::{Deserialize, Serialize};

use super::super::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetPower {
    id: i32,
    method: String,
    params: (types::Power, types::Effect, u32),
}

impl SetPower {
    pub fn new(
        power: types::Power,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Self {
        Self {
            id: 1,
            method: String::from("set_power"),
            params: (power, effect, duration.get_value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn set_power() {
        let msg = SetPower::new(
            types::Power::On,
            types::Effect::Smooth,
            types::Duration::new(500),
        );
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json =
            r#"{"id":1,"method":"set_power","params":["on","smooth",500]}"#;
        assert_eq!(msg_serial, json);
    }
}
