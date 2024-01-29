use crate::block::{Block, BlockError};
use crate::transaction::Transaction;
use ripemd::digest::generic_array::GenericArray;

use std::sync::Arc;

const MINING_REWARD: f32 = 10.0;

/// There should be only one blockchain instance per node
#[derive(Debug)]
pub struct Blockchain {
    address: String,
    chain: Vec<Block>,
    mempool: Vec<Arc<Transaction>>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(address: String, difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::genesis();
        chain.push(genesis_block);
        let mempool = vec![];
        let difficulty = difficulty;

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

    pub fn last_block(&self) -> Result<&Block, ChainError> {
        if self.chain.len() == 0 {
            return Err(ChainError::RetrieveBlockError(
                "No blocks in the chain".into(),
            ));
        }
        self.chain.last().ok_or(ChainError::RetrieveBlockError(
            "Failed to retrieve block ( `.last()` call failed ) ".into(),
        ))
    }

    /// For testing
    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone() // Deep copy
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

    pub fn proof_of_work(&self) -> Result<Block, ChainError> {
        let previous_hash = match self.last_block()?.hash_raw() {
            Ok(hash) => hash,
            Err(e) => return Err(e.into()),
        };

        let mut nonce = 0;

        let previous_hash_str = format!("{:x}", previous_hash);

        let mut guess_block = Block::create_from(self.mempool(), nonce, previous_hash_str);
        while Self::valid_proof(&guess_block, self.difficulty).is_err() {
            guess_block.increment_nonce();
        }
        Ok(guess_block)
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
    pub fn mine(&mut self) -> Result<&Block, ChainError> {
        let self_address = self.address();
        self.add_transaction("the_network", &self_address, 1.0);

        let new_block = self.proof_of_work()?;

        self.verify_and_add_block(new_block)
    }

    fn valid_proof(
        // nonce: i64,
        // previous_hash: GenericArray<u8, typenum::U32>,
        // transactions: Vec<Arc<Transaction>>,
        guess_block: &Block,
        difficulty: usize,
    ) -> Result<(), ChainError> {
        let zeros = "0".repeat(difficulty);

        // let hash = guess_block.hash();
        let hash = guess_block.hash()?;
        if hash.starts_with(&zeros) {
            return Ok(());
        }
        Err(ChainError::ValidationError(("Invalid proof").into()))
    }
    fn verify_block(&self, block: &Block) -> Result<(), ChainError> {
        let previous_block = self
            .chain
            .last()
            .ok_or(ChainError::RetrieveBlockError(
                "Failed to get last block".into(),
            ))?
            .clone();

        let now = crate::block::Block::generate_timestamp();

        if block.timestamp() < previous_block.timestamp() || block.timestamp() > now {
            return Err(ChainError::ValidationError("Invalid timestamp".into()));
        }

        let previous_block_hash = previous_block.hash()?;

        let previous_hash = block.previous_hash();

        if previous_block_hash != previous_hash {
            return Err(ChainError::ValidationError(
                "Previous hash does not match".into(),
            ));
        }

        Self::valid_proof(block, self.difficulty)
    }
    pub fn verify_and_add_block(&mut self, block: Block) -> Result<&Block, ChainError> {
        self.verify_block(&block)?;
        self.chain.push(block);

        self.mempool.clear();

        match self.chain.last() {
            Some(block) => Ok(block),
            None => Err(ChainError::RetrieveBlockError(
                "Failed to retrieve block".into(),
            )),
        }
    }

    // pub fn add_block(&mut self) -> Result<&Block, ChainError> {
    //     let previous_hash = self.last_block()?.hash()?;
    //     let nonce = 0;
    //     self.create_block(nonce, previous_hash)
    // }

    fn create_block(&mut self, nonce: i64, previous_hash: String) -> Block {
        let block = Block::create_from(self.mempool(), nonce, previous_hash);
        return block;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn cannot_insert_block_without_pow() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);
        let new_block = blockchain.last_block().unwrap().clone();

        assert!(blockchain.verify_and_add_block(new_block).is_err());
    }

    #[test]
    fn block_added_references_previous_block() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        for _ in 0..10 {
            blockchain.mine().unwrap();
            assert_eq!(
                blockchain.last_block().unwrap().hash().unwrap(),
                blockchain.chain()[blockchain.chain().len() - 1]
                    .hash()
                    .unwrap()
            );
        }
    }

    #[test]
    fn hashes_are_unique() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        for _ in 0..10 {
            blockchain.mine().unwrap();
        }

        let set: HashSet<_> = blockchain
            .chain()
            .iter()
            .map(|block| block.hash().unwrap())
            .collect();
        assert_eq!(blockchain.chain().len(), set.len());
    }

    #[test]
    fn mempool_empty_after_block_created() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        blockchain.add_transaction("sender_address", "recipient_address", 100.0); // there must be a way to avoid this..

        assert_eq!(blockchain.mempool().len(), 1);

        blockchain.mine().expect("Failed to mine");

        assert_eq!(blockchain.mempool().len(), 0);
    }
}

#[derive(Debug)]
pub enum ChainError {
    ValidationError(String),
    SerializeError(serde_json::Error),
    RetrieveBlockError(String),
}

impl From<BlockError> for ChainError {
    fn from(e: BlockError) -> Self {
        match e {
            BlockError::SerializeError(e) => ChainError::SerializeError(e),
        }
    }
}
