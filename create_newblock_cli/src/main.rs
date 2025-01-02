use sha2::{Sha256, Digest};    // Import the SHA256 hashing library.
use std::io::{self, Write};    // used for input/output operations.
use chrono::Utc; // For generating timestamps

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Block {
    index: u32,            // The block's number in the chain.
    timestamp: String,     // When the block was created.
    data: String,          // Arbitrary data stored in the block.
    previous_hash: String, // Hash of the previous block.
    hash: String,          // Hash of the current block (computed later).
}


fn create_block(index: u32, data: String, previous_hash: String) -> Block {

    // Captures the current UTC time and converts it to a string.
    let timestamp = Utc::now().to_string();
    let content_to_hash: String = format!("{}{}{}{}", index, data, previous_hash, timestamp);

    let mut hasher = Sha256::new();
    hasher.update(content_to_hash);

    let hash = format!("{:x}", hasher.finalize());

    Block {
        index,
        timestamp,
        data,
        previous_hash,
        hash,
    }
}
 
 //Creates the first block in the blockchain, called the "genesis block."
fn initialize_genesis_block() -> Block {
    create_block(0, "Genesis Block".to_string(), "0".to_string())
}


fn main(){
// we initialize the blockchain as a mutable vector thay contains only the genesis block
let mut blockchain: Vec<Block> = vec![initialize_genesis_block()];


loop {

    println!("==== Blockchain CLI ====");
    println!("1. Add a new block");
    println!("2. View blockchain");
    println!("3. Exit");


    // user input is handled here
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    //Creates a mutable string to store user input.
    let mut choice = String::new();

    //Reads user input from the terminal and stores it in choice.
    io::stdin().read_line(&mut choice).unwrap();

    //Removes any leading/trailing whitespace from the input.
    let choice = choice.trim();


    // user choice is handled here
    match choice {
        "1" => {
            print!("Enter data for the new block: ");
            io::stdout().flush().unwrap();
            let mut data = String::new();
            io::stdin().read_line(&mut data).unwrap();
            let data = data.trim().to_string();

            // An instance for the previous just added block
            let previous_block = blockchain.last().unwrap();

            let new_block = create_block(
                previous_block.index + 1,
                data,
                previous_block.hash.clone(),
            );

            println!("New block created: {:?}", new_block);
            blockchain.push(new_block);
        }

        "2" => {
                println!("Blockchain:");
                for block in &blockchain {
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