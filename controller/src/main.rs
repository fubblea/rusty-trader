use app_state::{create_trader, get_trader_state, AppState};
use log::info;
use simple_logger::SimpleLogger;

use common::{TraderConfig, TraderState};
mod app_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let mut app_state = AppState::new().await;

    app_state
        .mongo
        .add_config(TraderConfig {
            id: 1,
            name: "Config 1".to_string(),
        })
        .await
        .unwrap();

    create_trader(&mut app_state, 1, 1, None).await;
    create_trader(&mut app_state, 2, 1, None).await;

    info!("Listening to traders");
    loop {
        for (service_name, trader) in app_state.traders.iter() {
            info!(
                "Trader with id: {} is {:?}",
                trader.id,
                get_trader_state(service_name).unwrap_or(TraderState::NotAvailable)
            );
        }
    }
}
