use sha2::{Sha256, Digest};    // Import the SHA256 hashing library.
use hex;

#[derive(Debug)]
struct Block {
    data: String,
}

#[derive(Debug, PartialEq)]
enum BlockError {
    InvalidHashFormat,
    EmptyData,
}

fn validate_block(block: &Block) -> Result<(), BlockError> {
    // Check if the data is empty
    if block.data.is_empty() {
        return Err(BlockError::EmptyData);
    }

    // Compute the SHA-256 hash of the block data
    let hash = compute_hash(&block.data);

    // Check if the hash starts with "0000"
    if !hash.starts_with("0000") {
        return Err(BlockError::InvalidHashFormat);
    }

    Ok(())
}

fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    // Convert the hash result to a hexadecimal string
    hex::encode(result)
}

fn main() {
    let valid_block = Block {
        data: "Valid transaction data".to_string(),
    };

    let invalid_data_block = Block {
        data: "".to_string(),
    };

    let invalid_hash_block = Block {
        data: "Some valid data but wrong hash".to_string(),
    };

    match validate_block(&valid_block) {
        Ok(_) => println!("Valid block: {:?}", valid_block),
        Err(e) => println!("Error validating block: {:?}", e),
    }

    match validate_block(&invalid_data_block) {
        Ok(_) => println!("Valid block: {:?}", invalid_data_block),
        Err(e) => println!("Error validating block: {:?}", e),
    }

    match validate_block(&invalid_hash_block) {
        Ok(_) => println!("Valid block: {:?}", invalid_hash_block),
        Err(e) => println!("Error validating block: {:?}", e),
    }
}
