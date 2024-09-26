use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt;
use super::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, transactions: Vec<Transaction>, nonce: u64, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            nonce,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.nonce, self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        while &self.hash[..difficulty] != &"0".repeat(difficulty) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
