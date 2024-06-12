use common::{mongo::Mongo, Trader, TraderState};
use core::time::Duration;
use iceoryx2::prelude::*;
use log::{error, info};
use simple_logger::SimpleLogger;

use clap::Parser;

/// Create a trader with the specified config
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Trader id
    #[arg(short, long)]
    id: u32,

    /// Trader config
    #[arg(short, long)]
    config: u32,

    /// Trader service name
    #[arg(short, long)]
    service_name: String,

    /// Trader cycle time
    #[arg(short, long, default_value_t = 5)]
    cycle_time: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let args = Args::parse();

    info!(
        "Starting trader with id: {}, config: {}, service name: {}",
        args.id, args.config, args.service_name
    );

    let service_name = ServiceName::new(&args.service_name)?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TraderState>()?;

    let publisher = service.publisher().create()?;

    let trader = Trader::new(args.id, args.config, args.cycle_time);
    let mut mongo = Mongo::new(None).await?;

    while let Iox2Event::Tick = Iox2::wait(Duration::from_millis(args.cycle_time)) {
        let sample = publisher.loan_uninit()?;

        let state = step(&trader, &mut mongo).await?;

        let sample = sample.write_payload(state);
        sample.send()?;
    }

    Ok(())
}

async fn step(
    trader: &Trader,
    mongo: &mut Mongo,
) -> Result<TraderState, Box<dyn std::error::Error>> {
    match mongo.get_config(trader.config_id).await? {
        Some(_) => Ok(TraderState::Waiting),
        None => {
            error!("No config found for trader with id: {}", trader.id);
            Ok(TraderState::NotAvailable)
        }
    }
}
