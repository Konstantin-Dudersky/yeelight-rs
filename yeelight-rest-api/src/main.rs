use axum::{extract::Json, routing::post, Router};

use yeelight_protocol::types;
use yeelight_rust_sync_client::Bulb;

use yeelight_rest_api::models;

async fn power_on(Json(payload): Json<models::PowerOn>) {
    println!("{:?}", payload);
    let bulb = Bulb::new(&payload.address);
    bulb.set_power(types::Power::On, payload.effect, payload.duration)
        .unwrap();
}

async fn power_off(Json(payload): Json<models::PowerOn>) {
    let bulb = Bulb::new(&payload.address);
    bulb.set_power(
        types::Power::Off,
        payload.effect,
        types::Duration::new(1000),
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/power_on", post(power_on))
        .route("/power_off", post(power_off));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
