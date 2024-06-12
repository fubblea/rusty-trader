use std::collections::HashMap;
use std::time::Duration;

use common::mongo::Mongo;
use common::{Trader, TraderState};
use iceoryx2::prelude::*;
use iceoryx2::service::service_name::ServiceName;
use log::{error, info};

// Listening to traders at 200Hz
const CYCLE_TIME: Duration = Duration::from_millis(5);

#[derive(Debug)]
pub(crate) struct AppState {
    pub(crate) traders: HashMap<ServiceName, Trader>,
    pub(crate) mongo: Mongo,
}

impl AppState {
    pub(crate) async fn new() -> Self {
        Self {
            traders: HashMap::new(),
            mongo: Mongo::new(None).await.unwrap(),
        }
    }

    pub(crate) async fn add_trader(&mut self, trader: Trader) {
        let service_name = ServiceName::new(&trader.get_service_string()).unwrap();

        info!(
            "Adding trader with id: {}, service name: {}",
            &trader.id,
            &trader.get_service_string()
        );

        self.mongo.add_trader(trader.clone()).await.unwrap();
        self.traders.insert(service_name, trader);
    }
}

pub(crate) async fn create_trader(
    app_state: &mut AppState,
    id: u32,
    config: u32,
    cycle_time: Option<u64>,
) {
    let cycle_time = cycle_time.unwrap_or(CYCLE_TIME.as_millis() as u64);

    let new_trader = Trader::new(id, config, cycle_time);
    info!(
        "Creating trader with id: {}, config: {}, service name: {}",
        id,
        config,
        new_trader.get_service_string()
    );

    app_state.add_trader(new_trader).await;
}

pub(crate) fn get_trader_state(service_name: &ServiceName) -> Option<TraderState> {
    info!("Getting trader state using service name: {}", service_name);
    const MAX_ATTEMPTS: usize = 10;

    let service = zero_copy::Service::new(service_name)
        .publish_subscribe()
        .open_or_create::<TraderState>()
        .unwrap();

    let subscriber = service.subscriber().create().unwrap();

    let mut attempts = 0;
    while attempts < MAX_ATTEMPTS {
        attempts += 1;
        Iox2::wait(CYCLE_TIME);

        if let Some(sample) = subscriber.receive().unwrap() {
            return Some(*sample);
        }
    }

    error!("Failed to get trader state after {} attempts", attempts);
    None
}
