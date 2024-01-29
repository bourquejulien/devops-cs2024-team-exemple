use axum::http::StatusCode;
use axum::response::Html;
use serde_json::{Value, json};
use axum::{response::Json,};

pub async fn root() -> (StatusCode, Html<&'static str>) {
    let html_response = r#"
    <H1>Jungle:</H1>
    <a href="jungle">Here</a>
    "#;

    (StatusCode::NOT_FOUND, Html(html_response))
}

pub async fn get_health_status() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "response": "ok" })))
}
