use std::sync::Arc;

use axum::{extract::State, response::Html};

use crate::alpaca::Alpaca;

#[derive(Debug)]
enum TraderState {
    Waiting,
}

#[derive(Debug)]
pub struct Trader {
    alpaca: Alpaca,
    state: TraderState,
}

impl Trader {
    pub async fn new() -> Self {
        Self {
            alpaca: Alpaca::new().await,
            state: TraderState::Waiting,
        }
    }
}

pub async fn get_account(State(trader_bot): State<Arc<Trader>>) -> String {
    let account_details = &trader_bot.alpaca.get_account_details().await;
    serde_json::to_string_pretty(account_details).unwrap()
}

pub async fn get_state(State(trader_bot): State<Arc<Trader>>) -> Html<String> {
    let positions = &trader_bot.alpaca.get_open_positions().await;
    let orders = &trader_bot.alpaca.get_open_orders().await;
    let watchlist = trader_bot.alpaca.get_watchlist().await;

    let response = format!(
        "<body text=\"#ffffff\" style=\"background-color:black;\">
        <h1>Rusty Trader</h1>\n
        <h2>State:</h2>\n{:?}\n
        <h2>Open Positions:</h2>\n{}\n
        <h2>Open Orders:</h2>\n{}
        <h2>Watchlist:</h2>\n{}
        </body>",
        &trader_bot.state,
        serde_json::to_string_pretty(positions).unwrap(),
        serde_json::to_string_pretty(orders).unwrap(),
        serde_json::to_string_pretty(watchlist).unwrap(),
    );

    Html(response)
}
