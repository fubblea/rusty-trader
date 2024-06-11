#[derive(Debug)]
pub struct Trader {
    pub id: u64,
    config: u64,
}

impl Trader {
    pub fn new(id: u64, config: u64) -> Self {
        Self { id, config }
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
