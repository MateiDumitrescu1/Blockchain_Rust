use super::block::Block;
use super::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2,
            pending_transactions: Vec::new(),
            mining_reward: 100.0,
        };
        let genesis_block = Block::new(
            0,
            Blockchain::current_timestamp(),
            Vec::new(),
            0,
            String::from("0"),
        );
        blockchain.chain.push(genesis_block);
        blockchain
    }

    pub fn current_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction::new(String::from("System"), miner_address.clone(), self.mining_reward);
        self.pending_transactions.push(reward_tx);

        let mut block = Block::new(
            self.chain.len() as u64,
            Blockchain::current_timestamp(),
            self.pending_transactions.clone(),
            0,
            self.get_latest_block().hash.clone(),
        );
        block.mine_block(self.difficulty);
        self.chain.push(block);

        self.pending_transactions = Vec::new();
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn get_balance(&self, address: String) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.sender == address {
                    balance -= transaction.amount;
                }
                if transaction.recipient == address {
                    balance += transaction.amount;
                }
            }
        }
        balance
    }
}
