use axum::Json;

pub async fn upgrade() -> Json<String> {
    Json(String::from("Ok"))
}
