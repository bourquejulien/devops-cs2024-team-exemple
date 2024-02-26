use axum::{http::StatusCode, response::Response};
use axum::response::{Html, IntoResponse};
use serde_json::{json};
use axum::response::Json;

pub async fn root() -> (StatusCode, Response) {
    let html_response = r#"
    <H1>Jungle:</H1>
    <a href="jungle">Here</a>
    "#;

    (StatusCode::NOT_FOUND, Html(html_response).into_response())
}

pub async fn get_health_status() -> (StatusCode, Response) {
    (StatusCode::OK, Json(json!({ "response": "ok" })).into_response())
}
