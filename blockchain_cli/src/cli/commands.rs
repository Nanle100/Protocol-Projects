use std::io::{self, Write};

use crate::blockchain::chain::Blockchain;

pub fn run_cli() {
    let mut blockchain = Blockchain::new();

    loop {
        println!("\nBlockchain CLI");
        println!("1. Add Block");
        println!("2. View Blockchain");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter data for the new block:");
                let mut data = String::new();
                io::stdin().read_line(&mut data).unwrap();
                blockchain.add_block(data.trim().to_string());
                println!("Block added!");
            }
            "2" => {
                println!("Blockchain:");
                for block in blockchain.get_blocks() {
                    println!("{:?}", block);
                }
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
