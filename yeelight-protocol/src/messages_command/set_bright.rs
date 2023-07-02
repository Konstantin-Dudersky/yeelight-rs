use serde::{Deserialize, Serialize};

use super::super::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetBright {
    id: i32,
    method: String,
    params: (u8, types::Effect, u32),
}

impl SetBright {
    pub fn new(
        brightness: types::Brightness,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Self {
        Self {
            id: 1,
            method: String::from("set_bright"),
            params: (brightness.get_value(), effect, duration.get_value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn new() {
        let msg = SetBright::new(
            types::Brightness::new(50),
            types::Effect::Smooth,
            types::Duration::new(500),
        );
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json =
            r#"{"id":1,"method":"set_bright","params":[50,"smooth",500]}"#;
        assert_eq!(msg_serial, json);
    }
}
