# Modular Blockchain

A simple modular blockchain project built with Rust. This project demonstrates how to organize a Rust application using modules, crates, and workspaces. It includes basic implementations for blocks and transactions to help you learn project structure and modular code organization.

---

## **Features**
- **Block Logic:** Handles the structure and functionality of a blockchain block.
- **Transaction Logic:** Manages the creation and storage of transactions.
- **Modular Design:** Uses separate modules for block and transaction logic.
- **Hashing with SHA-256:** Calculates the hash of each block to ensure integrity.
- **Dependency Management:** Demonstrates the use of `Cargo.toml` to manage external crates.

---

## **Project Structure**
 ```plaintext
modular_blockchain/
├── Cargo.toml       # Project dependencies and configuration
├── src/
│   ├── block.rs     # Block-related logic
│   ├── transaction.rs # Transaction-related logic
│   ├── lib.rs       # Library file exposing the modules
│   ├── main.rs      # Entry point for the application

---


HOW TO RUN
Prerequisites
1. install rust.


STEPS
1. Clone this repository:
    ```
    git clone https://github.com/your-username/modular_blockchain.git
cd modular_blockchain

2. Install dependencies:
    cargo build

3. Run the application:
    cargo run


Noted: 
Dependencies
sha2: A Rust crate for SHA-256 hashing.

[dependencies]
sha2 = "0.10"
