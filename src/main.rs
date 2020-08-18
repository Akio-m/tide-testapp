
mod rest;
use rest::system::{post2response, ping};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/ping").get(ping);
    app.at("/ping").post(post2response);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}