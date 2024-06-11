use app_state::AppState;
use core::time::Duration;
use iceoryx2::prelude::*;
use log::info;
use simple_logger::SimpleLogger;
use std::process::Command;

use common::{Trader, TraderState};
mod app_state;

const CYCLE_TIME: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    let mut app_state = AppState::new();

    create_trader(&mut app_state, 123, 456).await;
    // create_trader(&mut app_state, 678, 890).await;

    info!("Listening to traders");
    loop {
        for (service_name, trader) in app_state.traders.iter() {
            println!(
                "Trader with id: {} is {:?}",
                trader.id,
                get_trader_state(service_name)
                    .await
                    .unwrap_or(TraderState::NotAvailable)
            );
        }
    }
}

async fn create_trader(app_state: &mut AppState, id: u64, config: u64) {
    let new_trader = Trader::new(id, config);
    info!(
        "Creating trader with id: {}, config: {}, service name: {}",
        id,
        config,
        new_trader.get_service_string()
    );

    Command::new("./trader")
        .arg("--id")
        .arg(format!("{}", id))
        .arg("--config")
        .arg(format!("{}", config))
        .arg("--service-name")
        .arg(new_trader.get_service_string())
        .spawn()
        .expect("Failed to create trader");

    app_state.add_trader(new_trader);
}

async fn get_trader_state(service_name: &ServiceName) -> Option<TraderState> {
    info!("Getting trader state using service name: {}", service_name);

    let service = zero_copy::Service::new(service_name)
        .publish_subscribe()
        .open_or_create::<TraderState>()
        .unwrap();

    let subscriber = service.subscriber().create().unwrap();

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        if let Some(sample) = subscriber.receive().unwrap() {
            return Some(*sample);
        }
    }

    None
}
