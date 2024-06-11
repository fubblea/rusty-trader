#[derive(Debug)]
#[repr(C)]
pub struct Trader {
    id: u64,
    config: u64,
}

impl Trader {
    pub fn new(id: u64, config: u64) -> Self {
        Self { id, config }
    }
}
