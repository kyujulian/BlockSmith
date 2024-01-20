use crate::transaction::Transaction;
use serde::Serialize;
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug, Clone)]
pub struct Block {
    timestamp: i64,
    nonce: i64,
    previous_hash: String,
    transactions: Vec<Transaction>,
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

    pub fn hash(&self) -> String {
        let block_json = serde_json::to_string(&self).expect("Failed to Serialize Struct");

        sha2::Sha256::digest(block_json.as_bytes())
            .iter()
            .map(|byte| format!("{:x}", byte))
            .collect::<String>()
    }

    pub fn new(transactions: Vec<Transaction>, nonce: i64, previous_hash: String) -> Self {
        let timestamp = Self::generate_timestamp();

        Self {
            timestamp,
            nonce,
            previous_hash,
            transactions,
        }
    }

    fn generate_timestamp() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_secs() as i64;
    }
}
