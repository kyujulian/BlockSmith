use std::net::TcpListener;

use api;
use chain;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    chain::run().unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    api::run(listener)?.await
}
