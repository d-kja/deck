mod core;
mod integrations;

use core::{
    deck::Deck, server::{health, icon}, websocket::{upgrade},
    image::DeckImage
};
use std::{env, error::Error, sync::Arc};

use axum::{Router, routing, serve};
use dotenv::dotenv;
use serde_json::Value;
use tokio::{
    net::TcpListener,
    sync::{
        Mutex,
        broadcast::{self, Receiver, Sender},
    },
};
use tracing::info;

struct Context {
    deck: Deck,
    image: DeckImage,
    transmitter: Sender<Value>,
    receiver: Receiver<Value>,
}

type ContextType = Arc<Mutex<Context>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let deck_image = DeckImage::new();
    let deck = Deck::new();
    deck.reset().await?;

    let listener_handle = deck.listen().await.expect("Unable to instanciate a new thread to handle the button presses");
    // let animations_handle = deck.animate().await.expect("Unable to instanciate a new thread to handle the animations");

    let (tx, rx) = broadcast::channel::<Value>(100);

    let state = Arc::new(Mutex::new(Context {
        deck,
        image: deck_image,
        receiver: rx,
        transmitter: tx,
    }));

    let app = Router::new()
        .route("/icon", routing::post(icon))
        .route("/health", routing::get(health))
        .route("/upgrade", routing::get(upgrade))
        .with_state(state);

    let addr = env::var("ADDR").expect("HTTP address variable not found");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", &addr);

    serve(listener, app).await?;

    listener_handle.await?;
    // animations_handle.await?;

    deck_image.shutdown();
    Ok(())
}
