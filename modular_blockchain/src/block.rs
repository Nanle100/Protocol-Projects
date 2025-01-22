//Block structure (e.g., index, timestamp, transactions, hash, and previous hash).
//A method to create a new block.
//A method to calculate the block’s hash.

use crate::transaction::Transaction;
use sha2::{Sha256, Digest};


#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: String, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash
        );
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
