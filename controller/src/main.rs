use app_state::{create_trader, get_trader_state, AppState};
use log::info;
use simple_logger::SimpleLogger;

use common::TraderState;
mod app_state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    let mut app_state = AppState::new();

    create_trader(&mut app_state, 123, 456);
    create_trader(&mut app_state, 678, 890);

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
