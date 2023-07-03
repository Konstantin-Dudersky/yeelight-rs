use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::Serialize;
use serde_json::from_str as json_to_struct;
use serde_json::to_string as struct_to_json;

use super::super::constants;
use super::super::messages_result;

pub struct BulbProtocol {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl BulbProtocol {
    pub fn new(address: &Ipv4Addr) -> Self {
        Self {
            address: address.clone(),
            port: constants::BULB_PORT,
        }
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.address.to_string(), self.port)
    }

    pub fn get_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.address), self.port)
    }

    pub fn serialize<T>(&self, message: T) -> Result<Vec<u8>, String>
    where
        T: Serialize,
    {
        let value = match struct_to_json(&message) {
            Ok(value) => value,
            Err(err) => {
                let err = format!("Ошибка сериализации: {}", err.to_string());
                return Err(err);
            }
        };
        let value = format!("{}\r\n", value);
        let value = value.as_bytes().to_vec();
        Ok(value)
    }

    pub fn deserialize(
        &self,
        message: &Vec<u8>,
    ) -> Result<Vec<String>, String> {
        let value = String::from_utf8_lossy(message)
            .replace("\0", "")
            .replace("\r\n", "");

        if let Ok(response) = json_to_struct::<messages_result::Error>(&value) {
            Err(String::from(response.error.message))
        } else if let Ok(response) =
            json_to_struct::<messages_result::Result>(&value)
        {
            Ok(response.result)
        } else {
            let err = format!("Пришел ответ, невозможно распознать: {}", value);
            Err(String::from(err))
        }
    }
}
