# Blockchain CLI in Rust

This project is a simple **blockchain simulation** built in Rust. It implements a basic blockchain data structure with essential features like creating blocks, maintaining a chain, and interacting with the blockchain via a command-line interface (CLI). The project uses the SHA256 hashing algorithm for block integrity and includes timestamps for each block.

## Features
- **Genesis Block Initialization**: Automatically creates the first block in the blockchain.
- **Add New Blocks**: Users can append new blocks to the blockchain via the CLI.
- **View Blockchain**: Displays the full chain of blocks, showing their structure and data.
- **Immutable Data**: Each block includes a hash that depends on its contents and the hash of the previous block, ensuring integrity.

## How It Works
1. **Genesis Block**: The first block (`index 0`) is initialized with predefined data.
2. **Adding Blocks**: 
   - Users enter data for the new block.
   - The program calculates the hash based on the data, timestamp, and the previous block's hash.
   - A new block is appended to the chain.
3. **Viewing Blockchain**: Prints all blocks in the chain, including their `index`, `timestamp`, `data`, `hash`, and `previous_hash`.

## Technologies Used
- **Rust**: The primary programming language.
- **`sha2` crate**: For SHA256 hashing.
- **`chrono` crate**: For generating UTC timestamps.
- **Standard I/O**: For CLI interaction.

## Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).
- Add the necessary crates:
  ```bash
  cargo add sha2 chrono


 ## Installation  
 1. Clone the repository:   
    ```bash
    git clone https://github.com/your-username/blockchain-cli-rust.git
    cd blockchain-cli-rust
2. Build the project:
    ```bash
    cargo build
3. Run the project:
    ```bash
    cargo run

## Usage
 **When you run the project, you'll see a menu:**

    ```bash
    ==== Blockchain CLI ====
    1. Add a new block
    2. View blockchain
    3. Exit

***Option 1: Add a new block. You'll be prompted to input the block's data.***

***Option 2: View the blockchain. Displays all blocks with their details.***

***Option 3: Exit the application.***

## Example
***ADDING A BLOCK***

***1. Run the program***

    ```bash
    cargo run


***2. Select 1 to add a new block***
***3. Input the block's data:***

    ```bash
    Enter data for the new block: First transaction

## Viewing the blockchain
***Select 2 to view blockhain:***

    ```bash
    Blockchain:
    Block { index: 0, timestamp: "...", data: "Genesis Block", ... }
    Block { index: 1, timestamp: "...", data: "First transaction", ... }






