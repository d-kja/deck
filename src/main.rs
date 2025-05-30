use core::entity::{PRODUCT_ID, VENDOR_ID};
use std::error::Error;

use mirajazz::device::{list_devices, new_hidapi};
use tracing::info;

mod core;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Setting up and reading devices.");

    let hidapi = new_hidapi()?;
    let devices = list_devices(&hidapi, &[VENDOR_ID]);

    info!("Found {:?} connected devices", devices.len());

    for (vendor_id, product_id, serial) in devices {
        if product_id != PRODUCT_ID {
            continue;
        }

        info!("Deck is connected using {}:{}", vendor_id, product_id);
    }

    Ok(())
}
