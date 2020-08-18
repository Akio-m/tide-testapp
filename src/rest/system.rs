use serde_json::json;
use serde::{Deserialize, Serialize};
use tide::Request;

pub async fn ping(_request: Request<()>) -> tide::Result<serde_json::Value> {
    Ok(json!({"ok" : "pong"}))
}

pub async fn post2response(mut _request: Request<()>) -> tide::Result<serde_json::Value> {
    let body: Ping = _request.body_json().await?;
    Ok(json!(&body))
}

#[derive(Deserialize, Serialize)]
struct Ping {
    value: String,
}