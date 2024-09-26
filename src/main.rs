mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut my_blockchain = Blockchain::new();

    // Create some transactions
    my_blockchain.add_transaction(Transaction::new(String::from("Alice"), String::from("Bob"), 50.0));
    my_blockchain.add_transaction(Transaction::new(String::from("Bob"), String::from("Charlie"), 25.0));

    // Mine pending transactions
    println!("Starting the miner...");
    my_blockchain.mine_pending_transactions(String::from("Miner1"));

    println!("Balance of Miner1: {}", my_blockchain.get_balance(String::from("Miner1")));

    // Validate the chain
    println!("Is blockchain valid? {}", my_blockchain.is_chain_valid());

    // Tampering with the blockchain, 
    // TODO finish checking chain validity
    // my_blockchain.chain[1].transactions[0].amount = 1000.0;
    // println!("Is blockchain valid after tampering? {}", my_blockchain.is_chain_valid());
}
