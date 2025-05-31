use axum::{Json, extract::State};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::{ContextType, core::deck::DeckEvent};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn websocket() -> Json<String> {
    Json(String::from("Ok"))
}

#[debug_handler]
pub async fn icon(State(context): State<ContextType>, Json(data): Json<Value>) -> &'static str {
    let device = context.lock().await;
    device.deck.emit(DeckEvent::TEST).await;

    println!("Data: {:?}", data);
    device.deck.test_keys().await.unwrap();

    "ok"
}
