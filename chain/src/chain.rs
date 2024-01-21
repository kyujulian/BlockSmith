use crate::block::Block;
use crate::transaction::Transaction;

/// There should be only one blockchain instance per node
pub struct Blockchain {
    address: String,
    chain: Vec<Block>,
    mempool: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(address: String, difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::default();
        chain.push(genesis_block);
        let mempool = Vec::new();

        Self {
            address,
            chain,
            mempool,
            difficulty,
        }
    }

    /// For testing
    pub fn mempool(&self) -> Vec<Transaction> {
        self.mempool.clone()
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// For testing
    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone()
    }

    pub fn add_block(&mut self) -> Option<&Block> {
        let previous_hash = self.last_block()?.hash();
        let nonce = 0; // todo: this should be calculated

        self.create_block(nonce, previous_hash)
    }

    pub fn add_transaction(
        &mut self,
        sender_address: String,
        recipient_address: String,
        value: i64,
    ) {
        let transaction = Transaction::new(sender_address, recipient_address, value);
        self.mempool.push(transaction);
    }

    pub fn proof_of_work(&self) -> i64 {
        let transactions = self.mempool.clone();
        let previous_hash = self.last_block().expect("Failed to get last block").hash();
        let mut nonce = 0;

        while !Self::valid_proof(
            nonce,
            previous_hash.clone(),
            transactions.clone(),
            self.difficulty,
        ) {
            nonce += 1;
        }
        nonce
    }

    pub fn mine(&mut self) -> Option<&Block> {
        let nonce = self.proof_of_work();
        let previous_hash = self.last_block()?.hash();

        self.create_block(nonce, previous_hash)
    }

    fn valid_proof(
        nonce: i64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> bool {
        let zeros = "0".repeat(difficulty as usize);
        let guess_block = Block::new(transactions, nonce, previous_hash);
        if guess_block.hash().starts_with(&zeros) {
            return true;
        }
        false
    }

    fn create_block(&mut self, nonce: i64, previous_hash: String) -> Option<&Block> {
        let block = Block::new(self.mempool.clone(), nonce, previous_hash);

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
        let mut blockchain = Blockchain::new(String::from("my_address"), 2);
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
        let mut blockchain = Blockchain::new(String::from("my_address"), 2);

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

        assert_ne!(first_block.hash(), last_block.hash());
    }

    #[test]
    fn mempool_empty_after_block_created() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 2);

        blockchain.add_transaction(
            String::from("sender_address"),
            String::from("recipient_address"),
            100,
        );

        assert_eq!(blockchain.mempool.len(), 1);

        blockchain.add_block();

        assert_eq!(blockchain.mempool.len(), 0);
    }
}
