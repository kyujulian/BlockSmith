use crate::transaction::Transaction;
use ripemd::digest::generic_array::GenericArray;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sha2::Digest;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: i64,
    nonce: i64,
    previous_hash: String,
    transactions: Vec<Rc<Transaction>>,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 4)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("previous_hash", &self.previous_hash)?;

        // Serialize each transaction by dereferencing the Rc.
        // This will serialize the data pointed to by the Rc, not the Rc itself.
        let transaction_data: Vec<&Transaction> =
            self.transactions.iter().map(|rc| &**rc).collect();
        state.serialize_field("transactions", &transaction_data)?;

        state.end()
    }
}

impl Block {
    pub fn default() -> Self {
        let timestamp = Self::generate_timestamp();
        let nonce = 0; // todo: this should be calculated
        let previous_hash = String::from(""); // todo: this should be calculated

        Self {
            timestamp,
            nonce,
            previous_hash,
            transactions: vec![],
        }
    }

    /// testing purposes
    pub fn previous_hash(&self) -> String {
        self.previous_hash.clone()
    }

    pub fn hash_raw(&self) -> GenericArray<u8, typenum::U32> {
        let block_json = serde_json::to_string(&self).expect("Failed to Serialize Struct");

        sha2::Sha256::digest(block_json.as_bytes())
    }

    pub fn hash(&self) -> String {
        let block_json = serde_json::to_string(&self).expect("Failed to Serialize Struct");
        let hash = sha2::Sha256::digest(block_json.as_bytes());
        hex::encode(hash)
    }
    pub fn new(transactions: Vec<Rc<Transaction>>, nonce: i64, previous_hash: String) -> Self {
        let timestamp = Self::generate_timestamp();

        Self {
            timestamp,
            nonce,
            previous_hash,
            transactions,
        }
    }

    pub fn transactions(&self) -> Vec<Rc<Transaction>> {
        self.transactions.clone()
    }

    fn generate_timestamp() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_secs() as i64;
    }
}

#[cfg(test)]
mod tests {
    use crate::block::Block;

    #[test]
    fn test_block_hash() {
        let block = Block::default();
        let hash = block.hash_raw();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_raw_hash_converted_is_same() {
        let block = Block::default();
        let hash_raw: String = block
            .hash_raw()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let hash = block.hash();
        assert_eq!(hash, hash_raw);
    }
}
