use axum::{Json, extract::State, response::NoContent};
use serde_json::Value;

use crate::ContextType;
use super::deck::DeckEvent;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn websocket() -> Json<String> {
    Json(String::from("Ok"))
}

pub async fn icon(State(context): State<ContextType>, Json(data): Json<Value>) -> NoContent {
    let device = context.lock().await;
    device.deck.emit(DeckEvent::TEST).await;

    device.deck.test_keys().await.expect("could not test keys");

    NoContent
}
