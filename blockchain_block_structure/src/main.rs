// Import the SHA256 hashing library.
use sha2::{Sha256, Digest};

// Allow the struct to be printed and cloned.
#[derive(Debug, Clone)] 
#[allow(dead_code)]
struct Block{
    index: u32,                        // The block's number in the chain.
    timestamp: String,                 //  When the block was created.
    data: String,                     //  Arbitrary data stored in the block.
    previous_hash: String,             // Hash of the previous block.
    hash: String,                       // Hash of the current block.
}


// Function to create a new block
fn create_block(index: u32, timestamp: String, data: String, previous_hash: String) -> Block {
    // Concatenate all the block fields into a single string for hashing.
    let content_to_hash = format!("{}{}{}{}", index, &timestamp, &data, &previous_hash);

    // Compute the hash of the block's content.
    let mut hasher = Sha256::new();
    hasher.update(content_to_hash);
    let hash = format!("{:x}", hasher.finalize()); // Convert hash bytes to a readable string.

    // Return a new Block with all fields populated.
    Block {
        index,
        timestamp,
        data,
        previous_hash,
        hash,
    }
}

fn main() {
    // Create the genesis block (the first block in the chain).
    let genesis_block = create_block(
        0,                              // Index of the first block.
        "2024-12-31 12:00:00".to_string(), // Arbitrary timestamp.
        "Genesis Block".to_string(),    // Initial data.
        "0".to_string(),                // No previous block, so previous_hash is "0".
    );

    // Print the genesis block.
    println!("{:?}", genesis_block);

    // Create a second block linked to the genesis block.
    let second_block = create_block(
        1,                              // Increment the index.
        "2024-12-31 12:05:00".to_string(), // New timestamp.
        "Second Block".to_string(),     // Data for the second block.
        genesis_block.hash.clone(),     // Use the hash of the genesis block as previous_hash.
    );

    // Print the second block.
    println!("{:?}", second_block);
}


