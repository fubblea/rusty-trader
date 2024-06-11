use core::time::Duration;
use iceoryx2::prelude::*;
use std::process::Command;

use common::Trader;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("./trader")
        .arg("--id")
        .arg("123")
        .arg("--config")
        .arg("456")
        .spawn()
        .expect("Failed to create trader");

    let service_name = ServiceName::new("My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<Trader>()?;

    let subscriber = service.subscriber().create()?;

    println!("Listening");

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        while let Some(sample) = subscriber.receive()? {
            println!("Received: {:?}", *sample);
        }
    }

    Ok(())
}
