use std::process::Command;

pub mod alpaca;
pub mod mongo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Trader {
    pub id: u32,
    pub config_id: u32,
    pub process_id: u32,
}

impl Trader {
    pub fn new(id: u32, config: u32, cycle_time: u64) -> Self {
        // NOTE: The trader binary is expected to be in the same directory as the controller
        let process_id = Command::new("./trader")
            .arg("--id")
            .arg(format!("{}", id))
            .arg("--config")
            .arg(format!("{}", config))
            .arg("--service-name")
            .arg(format!("Trader/{}", id))
            .arg("--cycle-time")
            .arg(format!("{}", cycle_time))
            .spawn()
            .expect("Failed to create trader")
            .id();

        Self {
            id,
            config_id: config,
            process_id,
        }
    }

    pub fn get_service_string(&self) -> String {
        format!("Trader/{}", self.id)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum TraderState {
    NotAvailable,
    Waiting,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraderConfig {
    pub id: u32,
    pub name: String,
}
