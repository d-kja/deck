use axum::{Json, extract::State, response::NoContent};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::ContextType;

#[debug_handler]
pub async fn health() -> &'static str {
    "ok"
}

pub async fn websocket() -> Json<String> {
    Json(String::from("Ok"))
}

pub async fn icon(State(context): State<ContextType>, Json(data): Json<Value>) -> NoContent {
    // let device = context.lock().await;

    NoContent
}
