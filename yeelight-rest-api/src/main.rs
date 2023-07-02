use axum::{routing::get, Router};

use yeelight_protocol::types;
use yeelight_rust_sync_client::Bulb;

async fn power_on() {
    let bulb = Bulb::new("192.168.1.104");
    bulb.set_power(
        types::Power::On,
        types::Effect::Smooth,
        types::Duration::new(1000),
    )
    .unwrap();
}

async fn power_off() {
    let bulb = Bulb::new("192.168.1.104");
    bulb.set_power(
        types::Power::Off,
        types::Effect::Smooth,
        types::Duration::new(1000),
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/power_on", get(power_on))
        .route("/power_off", get(power_off));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
