use std::sync::Arc;

use axum::{extract::State, response::Html};

use crate::alpaca::Alpaca;

#[derive(Debug)]
enum TraderState {
    Waiting,
}

pub struct Trader {
    alpaca: Alpaca,
    state: TraderState,
}

impl Trader {
    pub fn new() -> Self {
        Self {
            alpaca: Alpaca::new(),
            state: TraderState::Waiting,
        }
    }
}

pub async fn get_account(State(trader_bot): State<Arc<Trader>>) -> String {
    let account_details = &trader_bot.alpaca.get_account_details().await;
    serde_json::to_string_pretty(account_details).unwrap()
}

pub async fn get_state(State(trader_bot): State<Arc<Trader>>) -> Html<String> {
    let positions =
        serde_json::to_string_pretty(&trader_bot.alpaca.get_open_positions().await).unwrap();
    let orders = serde_json::to_string_pretty(&trader_bot.alpaca.get_open_orders().await).unwrap();

    let response = format!(
        "<body text=\"#ffffff\" style=\"background-color:black;\">
        <h1>Rusty Trader</h1>\n
        <h2>State:</h2>\n{:?}\n
        <h2>Open Positions:</h2>\n{}\n
        <h2>Open Orders:</h2>\n{}
        </body>",
        &trader_bot.state, positions, orders
    );

    Html(response)
}
