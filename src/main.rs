use std::sync::Arc;

use axum::{routing::get, Router};
use log::info;

mod alpaca;
mod trader;

use trader::Trader;

#[tokio::main]
async fn main() {
    env_logger::init();

    let trader_bot = Arc::new(Trader::new().await);

    let app = Router::new()
        .route("/", get(trader::get_state))
        .route("/account", get(trader::get_account))
        .with_state(trader_bot);

    let bind_address = "0.0.0.0:3000".to_string();
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    info!("Serving on {}", &bind_address);

    axum::serve(listener, app).await.unwrap();
}
