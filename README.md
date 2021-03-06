# Rust communication library for Lego Powered Up

[![](https://img.shields.io/github/license/sciguy16/lego-powered-up?style=plastic)](https://choosealicense.com/licenses/mpl-2.0/)
![](https://img.shields.io/github/workflow/status/sciguy16/lego-powered-up/build?style=plastic)
[![](https://img.shields.io/crates/v/lego-powered-up?style=plastic)](https://crates.io/crates/lego-powered-up)
[![](https://img.shields.io/docsrs/lego-powered-up?style=plastic)](https://docs.rs/lego-powered-up)


## Example

See the [examples](https://github.com/sciguy16/lego-powered-up/tree/main/examples) directory for more!

```rust
use lego_powered_up::notifications::Power;
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

    println!("Run motors");
    let mut motor_c = hub.port(lego_powered_up::hubs::Port::C)?;
    let mut motor_d = hub.port(lego_powered_up::hubs::Port::D)?;
    motor_c.start_speed(50, Power::Cw(50))?;
    motor_d.start_speed(50, Power::Cw(50))?;

    sleep(Duration::from_secs(3));

    println!("Stop motors");
    motor_c.start_speed(0, Power::Float)?;
    motor_d.start_speed(0, Power::Brake)?;

    println!("Disconnect from hub `{}`", hub.get_name());
    hub.disconnect()?;

    println!("Done!");

    Ok(())
}
```

## Contributing
Contributions are welcome, particularly in the following areas:
* Bug reports and feature requests
* Support for hubs other than the Technic Medium Hub (I don't have any other types to test with at the moment)
* Support for peripherals other than the simple motors and hub LEDs
* Good APIs to control e.g. motor position
* More examples to demonstrate cool things we can do
* Client implementation
* `#![no_std]` support (controller & client)
* Testing on/porting to non-linux operating systems, e.g. Windows & Mac

## Future goals
* Bluetooth/USB control of NXT bricks
* Control RCX bricks via the IR tower

## Architecture

Main components (tokio tasks):

* PoweredUp
  * Listener for Bluetooth device discovery notifications from btleplug
* HubManager
  * Owns the Peripherals corresponding to each hub
  * Listens for subscription messages and updates the stored hub & device states

Communication:
* Internal RPC structure
  * HubManager listens on a control channel
  * Requests down the control channel may include the sending half of a response channel

## License
This library is available under the terms of the [Mozilla Public License 2.0](https://choosealicense.com/licenses/mpl-2.0/).

The examples provided in the [examples](https://github.com/sciguy16/lego-powered-up/tree/main/examples) directory are dedicated to the [public domain](https://creativecommons.org/publicdomain/zero/1.0/)