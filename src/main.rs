mod core;

use core::entity::{KEY_COUNT, PRODUCT_ID, VENDOR_ID};
use mirajazz::device::{Device, list_devices, new_hidapi};

use owo_colors::OwoColorize;
use std::{error::Error, process::exit};
use tracing::{error, info, warn};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Setting up hidapi to connect with device.");
    let hidapi = match new_hidapi() {
        Ok(value) => value,
        Err(err) => panic!("Unable to create hidapi instance, reason: {:?}", err),
    };

    let devices = list_devices(&hidapi, &[VENDOR_ID]);
    info!("Found a total of {:?} connected devices", devices.len());

    let mut hid_device: Option<(u16, u16, String)> = None;
    for (vendor_id, product_id, serial) in devices {
        if product_id != PRODUCT_ID {
            continue;
        }

        info!(
            "Deck is connected using {:04X}:{:04X}",
            vendor_id, product_id
        );
        hid_device = Some((vendor_id, product_id, serial));
        break;
    }

    if hid_device.is_none() {
        panic!("Device not found!~");
    }

    let (vendor_id, product_id, serial) = hid_device.unwrap();
    let device = match Device::connect(
        &hidapi,
        vendor_id,
        product_id,
        &serial,
        false,
        false,
        KEY_COUNT as usize,
        0,
    ) {
        Ok(value) => value,
        Err(err) => panic!("Unable to connect to device, reason: {:?}", err),
    };

    let count = device.key_count();
    info!("Device has a total of {} keys", count);

    info!("Updating brightness to 100%");
    device.set_brightness(100)?;

    info!("Resetting all the buttons");
    device.clear_all_button_images()?;

    Ok(())
}
