use std::collections::HashMap;

use common::Trader;
use iceoryx2::service::service_name::ServiceName;
use log::info;

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
