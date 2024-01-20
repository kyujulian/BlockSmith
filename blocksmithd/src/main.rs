use chain;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(chain::run().unwrap())
}
