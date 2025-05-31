use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::{core::deck::DeckEvent, Context, ContextType};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn websocket() -> Json<String> {
    Json(String::from("Ok"))
}

#[debug_handler]
pub async fn placeholder(
    State(context): State<ContextType>,
    Json(data): Json<Value>
) -> &'static str {
    let device = context.lock().await;
    device.deck.emit(DeckEvent::TEST).await;

    println!("Data: {:?}", data);

    "ok"
}
