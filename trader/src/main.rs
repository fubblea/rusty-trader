use common::TraderState;
use core::time::Duration;
use iceoryx2::prelude::*;

use clap::Parser;

/// Create a trader with the specified config
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Trader id
    #[arg(short, long)]
    id: u64,

    /// Trader config
    #[arg(short, long)]
    config: u64,

    /// Trader service name
    #[arg(short, long)]
    service_name: String,
}

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let service_name = ServiceName::new(&args.service_name)?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TraderState>()?;

    let publisher = service.publisher().create()?;

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        let sample = publisher.loan_uninit()?;

        let payload = TraderState::Waiting;

        let sample = sample.write_payload(payload);
        sample.send()?;
    }

    Ok(())
}
