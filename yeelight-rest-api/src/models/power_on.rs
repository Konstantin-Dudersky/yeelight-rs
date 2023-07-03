use serde::Deserialize;
use std::net::Ipv4Addr;

use yeelight_protocol::types;

#[derive(Debug, Deserialize)]
pub struct PowerOn {
    pub address: Ipv4Addr,
    pub effect: types::Effect,
    #[serde(flatten)]
    pub duration: types::Duration,
}
