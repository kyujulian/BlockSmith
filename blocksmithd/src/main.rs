use api;
use chain;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    chain::run().unwrap();

    api::run().await
}
