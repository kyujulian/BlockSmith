use crate::block::Block;
use crate::transaction::Transaction;

use ripemd::digest::generic_array::GenericArray;
use std::sync::Arc;

/// There should be only one blockchain instance per node
#[derive(Debug)]
pub struct Blockchain {
    address: String,
    chain: Vec<Block>,
    mempool: Vec<Arc<Transaction>>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(address: String) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::default();
        chain.push(genesis_block);
        let mempool = vec![];
        let difficulty = 3;

        Self {
            address,
            chain,
            mempool,
            difficulty,
        }
    }

    /// For testing
    pub fn mempool(&self) -> Vec<Arc<Transaction>> {
        self.mempool.clone() // Shallow copy
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// For testing
    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone() // Deep copy
    }

    pub fn add_block(&mut self) -> Option<&Block> {
        let previous_hash = self.last_block()?.hash();
        let nonce = 0;

        self.create_block(nonce, previous_hash)
    }

    pub fn add_transaction(
        &mut self,
        sender_address: &str,
        recipient_address: &str,
        value: f32,
    ) -> &mut Self {
        let transaction = Transaction::new(
            String::from(sender_address),
            String::from(recipient_address),
            value,
        );

        self.mempool.push(Arc::new(transaction));
        self
    }

    pub fn proof_of_work(&self) -> i64 {
        let previous_hash = self
            .last_block()
            .expect("Failed to get last block")
            .hash_raw();
        let mut nonce = 0;

        while !Self::valid_proof(
            nonce,
            previous_hash.clone(),
            self.mempool(),
            self.difficulty,
        ) {
            nonce += 1;
        }
        nonce
    }

    pub fn get_balance(&self, address: &str) -> f32 {
        let mut balance = 0.0;

        for block in self.chain.iter() {
            for transaction in block.transactions().iter() {
                if transaction.sender_address() == address {
                    balance -= transaction.value();
                }
                if transaction.recipient_address() == address {
                    balance += transaction.value();
                }
            }
        }
        balance
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
    pub fn mine(&mut self) -> Option<&Block> {
        let self_address = self.address();
        self.add_transaction("the_network", &self_address, 1.0);
        let nonce = self.proof_of_work();
        let previous_hash = self.last_block()?.hash();

        self.create_block(nonce, previous_hash)
    }

    fn valid_proof(
        nonce: i64,
        previous_hash: GenericArray<u8, typenum::U32>,
        transactions: Vec<Arc<Transaction>>,
        difficulty: usize,
    ) -> bool {
        let zeros = "0".repeat(difficulty);

        let previous_hash_str = format!("{:x}", previous_hash);
        let guess_block = Block::new(transactions, nonce, previous_hash_str);

        // let hash = guess_block.hash();
        let hash = guess_block.hash();
        if hash.starts_with(&zeros) {
            return true;
        }
        false
    }

    fn create_block(&mut self, nonce: i64, previous_hash: String) -> Option<&Block> {
        let block = Block::new(self.mempool(), nonce, previous_hash);

        self.chain.push(block);
        self.mempool.clear();

        self.chain.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_added_references_previous_block() {
        let mut blockchain = Blockchain::new(String::from("my_address"));
        let first_block = blockchain
            .last_block()
            .expect("Last block not found 1")
            .clone();

        blockchain.add_block();

        println!("blockchain: {:?}", blockchain.chain());

        let last_block = blockchain
            .last_block()
            .expect("last block not found")
            .clone();

        blockchain.add_block();

        println!("hash_2: {:?}", last_block.hash());
        assert_eq!(first_block.hash(), last_block.previous_hash());
    }

    #[test]
    fn hashes_are_unique() {
        let mut blockchain = Blockchain::new(String::from("my_address"));

        let first_block = blockchain
            .last_block()
            .expect("Last block not found 1")
            .clone();

        blockchain.add_block();

        let last_block = blockchain
            .last_block()
            .expect("last block not found")
            .clone();

        blockchain.add_block();

        assert_ne!(first_block.hash_raw(), last_block.hash_raw());
    }

    #[test]
    fn mempool_empty_after_block_created() {
        let mut blockchain = Blockchain::new(String::from("my_address"));

        blockchain.add_transaction("sender_address", "recipient_address", 100.0); // there must be a way to avoid this..

        assert_eq!(blockchain.mempool().len(), 1);

        blockchain.add_block();

        assert_eq!(blockchain.mempool().len(), 0);
    }
}
