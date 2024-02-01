use std::sync::Arc;

use axum::{routing::get, Router};

mod alpaca;
mod trader;

use trader::Trader;

#[tokio::main]
async fn main() {
    let trader_bot = Arc::new(Trader::new());

    let app = Router::new()
        .route("/", get(trader::get_state))
        .route("/account", get(trader::get_account))
        .with_state(trader_bot);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
