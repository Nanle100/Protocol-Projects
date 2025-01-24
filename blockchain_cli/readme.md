# Blockchain CLI Tool

## Overview
This project is a simple **Blockchain Command-Line Interface (CLI)** tool written in Rust. It demonstrates the basic concepts of a blockchain, including creating blocks, maintaining an in-memory blockchain, and interacting with users via a CLI. The project is modular, scalable, and a great starting point for understanding blockchain development.

## Features
- **Genesis Block**: Automatically initializes the blockchain with a genesis block.
- **Add Blocks**: Users can add new blocks with custom data to the blockchain.
- **View Blockchain**: Display the current state of the blockchain, including all blocks and their details.
- **Data Integrity**: Blocks reference the hash of the previous block, ensuring chain integrity.

## Project Structure
```plaintext
blockchain_project/
├── Cargo.toml                # Project dependencies and metadata
├── src/
│   ├── main.rs               # Entry point of the application
│   ├── lib.rs                # Exposes project modules
│   ├── blockchain/           # Core blockchain logic
│   │   ├── mod.rs            # Blockchain module definition
│   │   ├── block.rs          # Block structure and methods
│   │   └── chain.rs          # Blockchain management
│   ├── cli/                  # Command-Line Interface logic
│   │   ├── mod.rs            # CLI module definition
│   │   └── commands.rs       # CLI commands implementation
│   └── utils/                # Utility functions
│       ├── mod.rs            # Utilities module definition
│       └── hash.rs           # Hashing utilities
├── tests/
│   └── integration_test.rs   # Integration tests
```

## Dependencies
This project uses the following Rust crates:
- **[chrono](https://crates.io/crates/chrono)**: For generating timestamps.
- **[sha2](https://crates.io/crates/sha2)**: For hashing block data.

Add these dependencies in `Cargo.toml`:
```toml
[dependencies]
chrono = "0.4"
sha2 = "0.10"
```

## Getting Started
### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).

### Setup
1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd blockchain_project
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the application:
   ```bash
   cargo run
   ```

### Usage
Once the application starts, you can:
1. **Add a Block**: Enter custom data to create and append a new block to the blockchain.
2. **View Blockchain**: Display all blocks with their details (index, timestamp, data, previous hash, and hash).
3. **Exit**: Quit the application.

### Example Interaction
```plaintext
Blockchain CLI
1. Add Block
2. View Blockchain
3. Exit
Enter your choice: 1
Enter data for the new block:
Example Data
Block added!

Blockchain CLI
1. Add Block
2. View Blockchain
3. Exit
Enter your choice: 2
Blockchain:
Block { index: 0, timestamp: "2025-01-23T12:34:56Z", data: "Genesis Block", previous_hash: "0", hash: "abc123..." }
Block { index: 1, timestamp: "2025-01-23T12:35:10Z", data: "Example Data", previous_hash: "abc123...", hash: "def456..." }
```

## Testing
Run the integration tests to ensure everything works as expected:
```bash
cargo test
```

## Future Improvements
- Persist blockchain to a file or database for state recovery.
- Implement a more secure hash mechanism with proof-of-work or proof-of-stake.
- Add a network layer for simulating distributed blockchains.
- Improve error handling and input validation.

## License
This project is licensed under the MIT License. See `LICENSE` for details.

## Acknowledgments
Special thanks to the Rust and blockchain communities for providing resources and inspiration for this project.

