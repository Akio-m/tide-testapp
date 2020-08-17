use serde_json::json;
use serde::{Deserialize, Serialize};
use tide::Request;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/ping").get(ping);
    app.at("/ping").post(post2response);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn ping(_request: Request<()>) -> tide::Result<serde_json::Value> {
    Ok(json!({"ok" : "pong"}))
}

async fn post2response(mut _request: Request<()>) -> tide::Result<serde_json::Value> {
    let body: Ping = _request.body_json().await?;
    Ok(json!(&body))
}

#[derive(Deserialize, Serialize)]
struct Ping {
    value: String,
}