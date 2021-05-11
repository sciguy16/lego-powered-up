use crate::argparse::DevicesArgs;
use anyhow::Result;
use lego_powered_up::PoweredUp;

pub fn run(args: &DevicesArgs) -> Result<()> {
    let adapters = PoweredUp::devices()?;

    if let Some(idx) = args.index {
        if let Some(adapter) = adapters.iter().nth(idx) {
            println!("Showing 1 Bluetooth device:");
            lego_powered_up::print_adapter_info(idx, &adapter)?;
        } else {
            println!("No Bluetooth device found");
        }
        return Ok(());
    }

    if adapters.is_empty() {
        println!("No Bluetooth device found");
    } else {
        println!("Showing {} available Bluetooth devices:", adapters.len());
        for (idx, dev) in adapters.iter().enumerate() {
            lego_powered_up::print_adapter_info(idx, &dev)?;
        }
    }
    Ok(())
}