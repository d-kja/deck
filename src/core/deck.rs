use std::{error::Error, sync::Arc, time::Duration};

use elgato_streamdeck::{asynchronous::list_devices_async, info::Kind, new_hidapi, AsyncStreamDeck};
use image::open;
use tokio::sync::Mutex;
use tracing::{error, info};

pub const VENDOR_ID: u16 = 0x0300;
pub const PRODUCT_ID: u16 = 0x1010;

pub const KEY_COUNT: u8 = 18;
// pub const FILE_FORMAT: ImageFormat = ImageFormat {
//     mode: ImageMode::JPEG,
//     size: (90, 90),
//     rotation: ImageRotation::Rot90,
//     mirror: ImageMirroring::Both,
// };

pub enum DeckEvent {
    TEST,
}

pub struct Deck {
    device: Arc<Mutex<AsyncStreamDeck>>,
    kind: Kind
}

impl Deck {
    pub fn new() -> Self {
        info!("Setting up hidapi to connect with device.");
        let hidapi = match new_hidapi() {
            Ok(value) => value,
            Err(err) => panic!("Unable to create hidapi instance, reason: {:?}", err),
        };

        let devices = list_devices_async(&hidapi, false);
        info!("Found a total of {:?} connected devices", devices.len());

        let mut device = None;
        for (kind, serial) in devices {
            info!("Deck is connected using {:?}:{:?}", kind, serial);
            let instance = AsyncStreamDeck::connect(&hidapi, kind, &serial);
            if let Err(value) = instance {
                error!("Unable to connect to device, {}", value);
                continue;
            }

            device = Some((instance.unwrap(), kind));
            break;
        }

        if device.is_none() {
            panic!("No device found.");
        }

        let (device, kind) = device.unwrap();
        let device = Arc::new(Mutex::new(device));
        Self { device, kind }
    }

    pub async fn reset(&self) -> Result<(), Box<dyn Error>> {
        let device = self.device.lock().await;

        let background = open("assets/icons/samples/background.png")?;
        device.set_logo_image(background).await?;
        info!("Background updated");

        device.set_brightness(75).await?;
        info!("Updated brightness to 75%");

        device.clear_all_button_images().await?;
        info!("Reset all the buttons");

        Ok(())
    }

    // pub async fn listen(&self) -> Result<(), Box<dyn Error>> {}

    pub async fn emit(&self, event: DeckEvent) {
        match event {
            DeckEvent::TEST => info!("Event emitted!"),
        }
    }

    // pub async fn test_keys(&self) -> Result<(), Box<dyn Error>> {
    //     let device = self.device.as_ref();
    //
    //     for idx in 0..device.key_count() {
    //         let img = idx % 10;
    //         let path = format!("assets/icons/samples/{}.png", img + 1);
    //
    //         let placeholder = open(path)?;
    //         device.set_button_image(idx as u8, FILE_FORMAT, placeholder)?;
    //     }
    //
    //     device.flush()?;
    //     Ok(())
    // }
}
