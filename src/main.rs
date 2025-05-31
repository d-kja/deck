#![forbid(unsafe_code)]

mod core;

use core::{
    deck::Deck,
    routes::{health, placeholder, websocket},
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
    deck: Arc<Deck>,
    transmitter: Sender<Value>,
    receiver: Receiver<Value>,
}

type ContextType = Arc<Mutex<Context>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let deck = Arc::new(Deck::new());
    let deck_ref = deck.clone();

    deck.reset().await?;

    let (tx, rx) = broadcast::channel::<Value>(100);

    let state = Arc::new(Mutex::new(Context {
        deck: deck_ref,
        receiver: rx,
        transmitter: tx,
    }));

    let app = Router::new()
        .route("/health", routing::get(health))
        .route("/ws", routing::get(websocket))
        .route("/placeholder", routing::post(placeholder))
        .with_state(state);

    let addr = env::var("ADDR").expect("HTTP address variable not found");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", &addr);

    serve(listener, app).await?;
    deck.shutdown().await?;

    Ok(())
}
