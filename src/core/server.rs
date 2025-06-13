use axum::{Json, extract::State, response::NoContent};
use serde_json::Value;

use super::deck::DeckEvent;
use crate::ContextType;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn icon(State(context): State<ContextType>, Json(data): Json<Value>) -> NoContent {
    let device = context.lock().await;

    let background_path = "assets/background/default.png";
    let tiles = device.image.crop_grid(background_path);

    NoContent
}

pub async fn test_icon(State(context): State<ContextType>, Json(data): Json<Value>) -> NoContent {
    let device = context.lock().await;

    device.deck.emit(DeckEvent::TEST).await;
    device.deck.test_keys().await.expect("could not test keys");

    NoContent
}
