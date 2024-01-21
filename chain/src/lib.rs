use crate::chain::Blockchain;

pub mod block;
pub mod chain;
pub mod transaction;
pub mod wallet;

pub const MINING_DIFFICULTY: usize = 3;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_chain = Blockchain::new(String::from("my_address"), MINING_DIFFICULTY);

    my_chain.add_transaction(String::from("Alice"), String::from("Bob"), 100);

    my_chain.add_transaction(String::from("Bob"), String::from("Carol"), 20);

    println!("{:#?} {:#?}", my_chain.chain(), my_chain.mempool());

    my_chain.mine();

    println!("{:#?} {:#?}", my_chain.chain(), my_chain.mempool());
    my_chain.add_transaction(String::from("Bob"), String::from("Carol"), 20);

    println!("{:#?} {:#?}", my_chain.chain(), my_chain.mempool());

    my_chain.mine();

    println!("{:#?} {:#?}", my_chain.chain(), my_chain.mempool());
    Ok(())
}
