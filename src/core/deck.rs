use std::{error::Error, sync::Arc};

use elgato_streamdeck::{
    AsyncStreamDeck, DeviceStateUpdate,
    asynchronous::{AsyncDeviceStateReader, list_devices_async},
    info::Kind,
    new_hidapi,
};
use image::open;
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{error, info, warn};

pub enum DeckEvent {
    TEST,
    PLAY,
    PAUSE,
    NEXT,
    PREVIOUS,
}

pub struct Deck {
    pub kind: Kind,
    pub device: Arc<Mutex<AsyncStreamDeck>>,
    pub reader: Arc<AsyncDeviceStateReader>,
    pub size: (usize, usize),
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
            info!("Connection for {:?} found, attaching via hidapi", kind);

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
        let reader = device.get_reader();
        let size = device.kind().lcd_strip_size().unwrap_or((90, 90));

        let device = Arc::new(Mutex::new(device));

        Self {
            device,
            reader,
            kind,
            size,
        }
    }

    pub async fn reset(&self) -> Result<(), Box<dyn Error>> {
        let device = self.device.lock().await;

        let background = open("assets/background/default.jpg")?;

        device.set_logo_image(background).await?;
        info!("Background updated");

        device.set_brightness(50).await?;
        info!("Updated brightness to 50%");

        std::thread::sleep(std::time::Duration::from_millis(200));

        for tile_idx in 0..self.kind.key_count() {
            let path = format!("assets/icons/actions/action_{}.png", tile_idx);
            let tile = open(path);

            if tile.is_err() {
                continue;
            }

            let tile = tile?;
            device.set_button_image(tile_idx, tile).await?;

            info!("Updated button {}", tile_idx);
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
        device.flush().await?;
        info!("Flushed display");

        Ok(())
    }

    pub async fn listen(&self) -> Result<JoinHandle<()>, Box<dyn Error>> {
        let kind = self.kind;
        let reader = self.reader.clone();

        let handle = tokio::spawn(async move {
            info!("Setting up an event listener.");

            'emitter: loop {
                let updates = match reader.read(10.0).await {
                    Ok(value) => {
                        warn!("Event found, {:?}", value);
                        value
                    }
                    Err(err) => {
                        error!("An error ocurred when trying to handle an event, {:?}", err);
                        panic!("Listener is deadge, see you next time 🫡");
                    }
                };

                for update in updates {
                    match update {
                        DeviceStateUpdate::ButtonDown(key) => {
                            info!("Button {:?} down", key);
                        }
                        DeviceStateUpdate::ButtonUp(key) => {
                            info!("Button {:?} up", key);
                            if key == kind.key_count() - 1 {
                                break 'emitter;
                            }
                        }

                        _ => {
                            warn!("Unhandled event {:?}", update);
                        }
                    }
                }
            }

            error!("Closing event listener");
        });

        Ok(handle)
    }

    // TODO: create a configuration file to indicate who needs to be animated
    // pub async fn animate(&self) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }

    pub async fn emit(&self, event: DeckEvent) {
        match event {
            DeckEvent::PLAY => info!("Event play was emitted"),
            DeckEvent::PAUSE => info!("Event pause was emitted"),

            DeckEvent::NEXT => info!("Event next was emitted!"),
            DeckEvent::PREVIOUS => info!("Event previous was emitted!"),

            DeckEvent::TEST => info!("Health check event emitted!"),
        }
    }

    pub async fn test_keys(&self) -> Result<(), Box<dyn Error>> {
        let device = self.device.lock().await;
        let key_count = self.kind.key_count();

        for idx in 0..key_count {
            let img = idx % 10;
            let path = format!("assets/icons/samples/{}.png", img + 1);

            let placeholder = open(path)?;
            device.set_button_image(idx as u8, placeholder).await?;
        }

        device.flush().await?;
        Ok(())
    }
}
