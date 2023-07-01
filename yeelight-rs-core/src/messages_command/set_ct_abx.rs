use serde::{Deserialize, Serialize};

use super::super::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetCtAbx {
    id: i32,
    method: String,
    params: (u32, types::Effect, u32),
}

impl SetCtAbx {
    pub fn new(
        ct_value: types::ColorTemperature,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Self {
        Self {
            id: 1,
            method: String::from("set_ct_abx"),
            params: (ct_value.get_value(), effect, duration.get_value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn set_ct_abx() {
        let msg = SetCtAbx::new(
            types::ColorTemperature::new(3500),
            types::Effect::Smooth,
            types::Duration::new(500),
        );
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json =
            r#"{"id":1,"method":"set_ct_abx","params":[3500,"smooth",500]}"#;
        assert_eq!(msg_serial, json);
    }
}
