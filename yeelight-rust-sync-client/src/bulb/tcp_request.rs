use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn tcp_request(
    socket_addr: &SocketAddr,
    request: &Vec<u8>,
) -> Result<Vec<u8>, String> {
    let mut stream = tcp_connect(socket_addr)?;
    tcp_write(&mut stream, request)?;
    let response = tcp_read(&mut stream)?;
    Ok(response)
}

fn tcp_connect(socket_addr: &SocketAddr) -> Result<TcpStream, String> {
    match TcpStream::connect_timeout(socket_addr, Duration::from_millis(1000)) {
        Ok(value) => Ok(value),
        Err(err) => {
            let err = format!("Ошибка при подключении: {}", err.to_string());
            return Err(err);
        }
    }
}

fn tcp_write(stream: &mut TcpStream, request: &[u8]) -> Result<(), String> {
    match stream.write(request) {
        Ok(_) => Ok(()),
        Err(err) => {
            let err = format!("Ошибка при записи в tcp: {}", err.to_string());
            return Err(err);
        }
    }
}

fn tcp_read(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut response = [0; 128];
    match stream.read(&mut response) {
        Ok(_) => (),
        Err(err) => {
            let err = format!("Ошибка при чтении из tcp: {}", err.to_string());
            return Err(err);
        }
    };
    Ok(response.to_vec())
}
