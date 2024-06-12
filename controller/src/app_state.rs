use std::time::Duration;
use std::{collections::HashMap, process::Command};

use common::{Trader, TraderState};
use iceoryx2::prelude::*;
use iceoryx2::service::service_name::ServiceName;
use log::{error, info};

// Listening to traders at 200Hz
const CYCLE_TIME: Duration = Duration::from_millis(5);

#[derive(Debug)]
pub(crate) struct AppState {
    pub(crate) traders: HashMap<ServiceName, Trader>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            traders: HashMap::new(),
        }
    }

    pub(crate) fn add_trader(&mut self, trader: Trader) {
        let service_name = ServiceName::new(&trader.get_service_string()).unwrap();

        info!(
            "Adding trader with id: {}, service name: {}",
            &trader.id,
            &trader.get_service_string()
        );

        self.traders.insert(service_name, trader);
    }
}

pub(crate) fn create_trader(app_state: &mut AppState, id: u64, config: u64) {
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

pub(crate) fn get_trader_state(service_name: &ServiceName) -> Option<TraderState> {
    info!("Getting trader state using service name: {}", service_name);
    const MAX_ATTEMPTS: usize = 10;

    let service = zero_copy::Service::new(service_name)
        .publish_subscribe()
        .open_or_create::<TraderState>()
        .unwrap();

    let subscriber = service.subscriber().create().unwrap();

    let mut attempts = 0;
    while attempts <= MAX_ATTEMPTS {
        attempts += 1;
        Iox2::wait(CYCLE_TIME);

        if let Some(sample) = subscriber.receive().unwrap() {
            return Some(*sample);
        }
    }

    error!("Failed to get trader state after {} attempts", attempts);
    None
}
