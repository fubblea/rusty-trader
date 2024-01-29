use std::sync::Arc;

use axum::{routing::get, Router};

mod alpaca;
mod trader;

use alpaca::Alpaca;

#[tokio::main]
async fn main() {
    let alpaca_client = Arc::new(Alpaca::new());

    let app = Router::new()
        .route("/", get(alpaca::get_account))
        .with_state(alpaca_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
