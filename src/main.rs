use core::entity::VENDOR_ID;
use std::error::Error;

use mirajazz::device::{list_devices, new_hidapi};

mod core;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting up...");

    let hidapi = new_hidapi()?;
    let devices = list_devices(&hidapi, &[VENDOR_ID]);

    for (vendor_id, product_id, serial) in devices {
        println!("[INFO] - Item found:\n - Vendor: {}\n - Product: {}\n - Serial: {}", vendor_id, product_id, serial);
    }

    Ok(())
}
