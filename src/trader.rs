use std::sync::Arc;

use axum::extract::State;

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

pub async fn get_state(State(trader_bot): State<Arc<Trader>>) -> String {
    let open_positions = &trader_bot.alpaca.get_open_positions().await;
    let positions = serde_json::to_string_pretty(open_positions).unwrap();

    format!(
        "State: {:?}\nOpen Positions: {}",
        &trader_bot.state, positions
    )
}
