use core::time::Duration;
use iceoryx2::prelude::*;

use common::Trader;

use clap::Parser;

/// Create a trader with the specified config
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    id: u64,

    /// Number of times to greet
    #[arg(short, long)]
    config: u64,
}

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let service_name = ServiceName::new("My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<Trader>()?;

    let publisher = service.publisher().create()?;

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        let sample = publisher.loan_uninit()?;

        let payload = Trader::new(args.id, args.config);
        println!("Sending : {:?}", &payload);

        let sample = sample.write_payload(payload);
        sample.send()?;
    }

    Ok(())
}
