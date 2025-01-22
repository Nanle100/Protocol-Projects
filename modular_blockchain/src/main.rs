// The main.rs file will use the block and transaction modules to simulate adding blocks to the blockchain.

use modular_blockchain::block::Block;
use modular_blockchain::transaction::Transaction;

fn main() {
    let transaction1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
    let transaction2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 20.0);

    let transactions = vec![transaction1, transaction2];

    let genesis_block = Block::new(0, "2025-01-22".to_string(), transactions, "0".to_string());
    
    println!("Genesis Block: {:#?}", genesis_block);
}
