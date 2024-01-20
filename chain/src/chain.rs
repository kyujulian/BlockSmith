use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    address: String,
    chain: Vec<Block>,
    mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(address: String) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::default();
        chain.push(genesis_block);
        let mempool = Vec::new();

        Self {
            address,
            chain,
            mempool,
        }
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// testing purposes
    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone()
    }

    pub fn add_block(&mut self) -> Option<&Block> {
        let previous_hash = self.last_block()?.hash();
        let nonce = 0; // todo: this should be calculated

        self.create_block(nonce, previous_hash)
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

        assert_eq!(first_block.hash(), last_block.previous_hash());
    }
}
