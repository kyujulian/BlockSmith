use crate::{chain::Blockchain, wallet::Wallet};

pub mod block;
pub mod chain;
pub mod transaction;
pub mod wallet;

pub const MINING_DIFFICULTY: usize = 4;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let wallet_a = Wallet::generate_new();
    let a_address = wallet_a.address();

    // let wallet_b = Wallet::generate_new();
    // let b_address = wallet_b.address();

    // let wallet_c = Wallet::generate_new();
    // let c_address = wallet_c.address();

    let mut my_chain = Blockchain::new(String::from("my_address"));

    println!("chain {:?}", my_chain);

    println!("Now mining");

    my_chain.mine().unwrap();

    println!("last block {:#?}", my_chain.last_block());

    // my_chain = my_chain.add_transaction(&a_address, &b_address, 100);

    // my_chain = my_chain.add_transaction(&b_address, &c_address, 20);

    // println!(
    //     "Chain: {:#?},  MemPool: {:#?}",
    //     my_chain.chain(),
    //     my_chain.mempool()
    // );

    // my_chain.mine();

    // println!(
    //     "Chain: {:#?}, MemPool: {:#?}",
    //     my_chain.chain(),
    //     my_chain.mempool()
    // );

    // my_chain.mine();

    // println!("{:#?} {:#?}", my_chain.chain(), my_chain.mempool());
    // println!("Balancd A: {:#?}", my_chain.get_balance(&a_address));
    // println!("Balancd B: {:#?}", my_chain.get_balance(&b_address));
    // println!("Balancd C: {:#?}", my_chain.get_balance(&c_address));
    Ok(())
}
