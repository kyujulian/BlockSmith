use api;
use chain;

fn main() -> std::io::Result<()> {
    chain::run().unwrap();

    api::run()
}
