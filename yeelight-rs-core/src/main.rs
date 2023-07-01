use std::io::prelude::*;
use std::net::TcpStream;

use yeelight_rs::Bulb;

fn main2() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.108:55443")?;

    // let req = b"{ \"id\": 1, \"method\": \"set_power\", \"params\":[\"on\", \"smooth\", 1000]}\r\n";

    let req = b"{ \"id\": 1, \"method\": \"set_power\", \"params\":[\"off\",\"smooth\", 1000]}\r\n";

    stream.write(req)?;
    let mut ans = [0; 128];
    stream.read(&mut ans)?;
    let ans = String::from_utf8_lossy(&ans)
        .replace("\0", "")
        .replace("\r\n", "");
    println!("{:?}", ans);

    if let Ok(response) = serde_json::from_str::<MessageResponseError>(&ans) {
        println!("{:?}", response);
    } else if let Ok(response) = serde_json::from_str::<MessageResponseOk>(&ans)
    {
        println!("{:?}", response);
    }

    let mut a = 0;

    Ok(())
}

fn main() {
    let bulb = Bulb::new("192.168.1.108");
}
