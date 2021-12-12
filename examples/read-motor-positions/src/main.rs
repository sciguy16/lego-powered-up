// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

use lego_powered_up::PoweredUp;
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    println!("Listening for hubs...");
    let pu = PoweredUp::init()?;
    let hub = pu.wait_for_hub()?;

    println!("Connecting to hub `{}`", hub.name);
    let hub = pu.create_hub(&hub)?;

    println!("Change the hub LED to green");
    let mut hub_led = hub.port(lego_powered_up::hubs::Port::HubLed)?;
    hub_led.set_rgb(&[0, 0xff, 0])?;

    println!("Read motor positions");
    let mut motor_a = hub.port(lego_powered_up::hubs::Port::A)?;
    let mut motor_b = hub.port(lego_powered_up::hubs::Port::B)?;

    let upper = 20;
    for count in 0..upper {
        let pos_a = motor_a.read_position()?;
        let pos_b = motor_b.read_position()?;
        println!("Reading {} of {}: a={}, b={}", count, upper, pos_a, pos_b);
        sleep(Duration::from_secs(1));
    }

    println!("Disconnect from hub `{}`", hub.get_name());
    hub.disconnect()?;

    println!("Done!");

    Ok(())
}
