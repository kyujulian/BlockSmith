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
        let genesis_block = Block::default();
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

    pub fn proof_of_work(&self) -> Result<i64, ChainError> {
        let previous_hash = match self.last_block()?.hash_raw() {
            Ok(hash) => hash,
            Err(e) => return Err(e.into()),
        };

        let mut nonce = 0;

        while Self::valid_proof(
            nonce,
            previous_hash.clone(),
            self.mempool(),
            self.difficulty,
        )
        .is_err()
        {
            nonce += 1;
        }
        Ok(nonce)
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
        let nonce = self.proof_of_work()?;

        let previous_hash = self.last_block()?.hash()?;
        let transaction =
            Transaction::new(String::from("Network"), self.address.clone(), MINING_REWARD);

        self.mempool.push(Arc::new(transaction));

        let new_block = self.create_block(nonce, previous_hash);

        self.verify_and_add_block(new_block)
    }

    fn valid_proof(
        nonce: i64,
        previous_hash: GenericArray<u8, typenum::U32>,
        transactions: Vec<Arc<Transaction>>,
        difficulty: usize,
    ) -> Result<(), ChainError> {
        let zeros = "0".repeat(difficulty);

        let previous_hash_str = format!("{:x}", previous_hash);
        let guess_block = Block::create_from(transactions, nonce, previous_hash_str);

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

        if block.timestamp() <= previous_block.timestamp() || block.timestamp() < now {
            return Err(ChainError::ValidationError("Invalid timestamp".into()));
        }

        let previous_block_hash = previous_block.hash()?;

        let previous_hash = block.hash()?;

        if previous_block_hash != previous_hash {
            return Err(ChainError::ValidationError(
                "Previous hash does not match".into(),
            ));
        }

        let previous_block_hash_raw = previous_block.hash_raw()?;

        Self::valid_proof(
            block.nonce(),
            previous_block_hash_raw,
            block.transactions(),
            self.difficulty,
        )
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

    #[test]
    fn cannot_insert_block_without_pow() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);
        let new_block = blockchain.last_block().unwrap().clone();

        assert!(blockchain.verify_and_add_block(new_block).is_err());
    }

    #[test]
    fn block_added_references_previous_block() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        blockchain.mine();
        let first_block = blockchain.last_block().unwrap().clone();

        blockchain.mine();
        let last_block = blockchain.last_block().unwrap().clone();

        assert_eq!(first_block.hash().unwrap(), last_block.previous_hash());
    }

    #[test]
    fn hashes_are_unique() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        blockchain.mine();
        let first_block = blockchain.last_block().unwrap().clone();

        blockchain.mine();
        let last_block = blockchain.last_block().unwrap().clone();

        assert_ne!(
            first_block.hash_raw().unwrap(),
            last_block.hash_raw().unwrap()
        );
    }

    #[test]
    fn mempool_empty_after_block_created() {
        let mut blockchain = Blockchain::new(String::from("my_address"), 3);

        blockchain.add_transaction("sender_address", "recipient_address", 100.0); // there must be a way to avoid this..

        assert_eq!(blockchain.mempool().len(), 1);

        blockchain.add_block().unwrap();

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
