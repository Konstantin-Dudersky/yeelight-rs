use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub id: i32,
    pub result: Vec<String>,
}
