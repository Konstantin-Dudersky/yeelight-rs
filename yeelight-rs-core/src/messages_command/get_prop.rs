use serde::{Deserialize, Serialize};

use super::all_properties::AllProperties;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProp {
    id: i32,
    method: String,
    params: Vec<AllProperties>,
}

impl GetProp {
    pub fn new(params: &Vec<AllProperties>) -> Self {
        Self {
            id: 1,
            method: String::from("get_prop"),
            // params: params.iter().map(|param| String::from(*param)).collect(),
            params: params.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn get_prop() {
        let msg = GetProp::new(&vec![AllProperties::Power]);
        let msg_serial = serde_json::to_string(&msg).unwrap();

        let json =
            r#"{"id":1,"method":"get_prop","params":["power","bright"]}"#;
        assert_eq!(msg_serial, json);
    }
}
