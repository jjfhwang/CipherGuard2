```markdown
# CipherGuard2

## Description

CipherGuard2 is a decentralized ledger implementation providing cryptographic primitives for secure data storage. It leverages Rust's memory safety and performance characteristics to build a robust and efficient platform for managing and securing data in a distributed environment. This project aims to provide a foundation for building secure and transparent applications where data integrity and immutability are paramount. The core focus is on providing a secure, auditable, and tamper-proof record of data transactions. CipherGuard2 is designed with modularity in mind, allowing developers to integrate its cryptographic and ledger functionalities into various applications, from supply chain management to secure voting systems.

## Features

*   **Decentralized Ledger:** Implements a distributed ledger system ensuring data immutability and transparency across multiple nodes. The ledger is based on a blockchain architecture with a consensus mechanism (currently Proof-of-Work, but pluggable).

*   **Advanced Cryptography:** Incorporates state-of-the-art cryptographic primitives, including SHA-256 hashing, elliptic curve cryptography (specifically, the secp256k1 curve), and digital signatures for secure data encryption and authentication.

*   **Secure Data Storage:** Provides a secure storage layer utilizing cryptographic techniques to protect data at rest, ensuring confidentiality and integrity even in the event of a node compromise. Data is encrypted before being written to the ledger.

*   **Transaction Management:**  Handles transaction processing, validation, and propagation across the network. Supports custom transaction types and allows for complex data structures to be securely stored on the ledger.

*   **Pluggable Consensus:** The consensus mechanism is designed to be pluggable, allowing for future upgrades and experimentation with different consensus algorithms (e.g., Proof-of-Stake).

## Installation

To install and run CipherGuard2, you will need to have Rust and Cargo installed. If you don't have them already, you can download them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/jjfhwang/CipherGuard2.git
    cd CipherGuard2
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

    This command compiles the project in release mode, optimizing for performance.

3.  **Install dependencies:**

    CipherGuard2 relies on several crates. Cargo automatically manages these dependencies during the build process. However, if you encounter any issues, ensure you have the following system dependencies installed (example for Debian/Ubuntu):

    ```bash
    sudo apt-get update
    sudo apt-get install build-essential libssl-dev pkg-config
    ```

    For other operating systems, consult the documentation for the specific crates used (listed in `Cargo.toml`). Common dependencies include `openssl`, `secp256k1`, and `tokio`.

4.  **Run the tests (optional):**

    ```bash
    cargo test
    ```

    This command executes the unit and integration tests to verify the correctness of the implementation.

## Usage

Here are some examples of how to use CipherGuard2. This assumes you have the project built and are familiar with basic Rust programming concepts.

```rust
// src/main.rs

use cipherguard2::ledger::{Ledger, Transaction};
use cipherguard2::crypto::{KeyPair, hash_data};

fn main() {
    // 1. Create a new Ledger instance.
    let mut ledger = Ledger::new();

    // 2. Generate a key pair for signing transactions.
    let keypair = KeyPair::generate();

    // 3. Create a transaction.
    let data = "Hello, CipherGuard2!".as_bytes();
    let data_hash = hash_data(data);
    let signature = keypair.sign(&data_hash);

    let transaction = Transaction {
        data: data.to_vec(),
        signature,
        public_key: keypair.public_key().to_vec(),
    };

    // 4. Add the transaction to the ledger.
    ledger.add_transaction(transaction);

    // 5. Mine a new block.
    ledger.mine_block();

    // 6. Verify the blockchain's integrity.
    if ledger.is_chain_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }

    // 7. Print the blockchain (for debugging purposes).
    println!("Blockchain: {:?}", ledger.blockchain);
}
```

```rust
// src/lib.rs (Example Ledger Module)

pub mod ledger {
    use chrono::Utc;
    use serde::{Serialize, Deserialize};
    use crate::crypto::{hash_data, verify_signature};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Transaction {
        pub data: Vec<u8>,
        pub signature: Vec<u8>,
        pub public_key: Vec<u8>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Block {
        pub timestamp: i64,
        pub transactions: Vec<Transaction>,
        pub previous_hash: String,
        pub hash: String,
        pub nonce: u64,
    }

    impl Block {
        pub fn new(timestamp: i64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
            let mut block = Block {
                timestamp,
                transactions,
                previous_hash,
                hash: String::new(),
                nonce: 0,
            };
            block.hash = block.calculate_hash();
            block
        }

        pub fn calculate_hash(&self) -> String {
            let data = format!(
                "{}{}{}{}",
                self.timestamp,
                self.transactions.iter().map(|t| format!("{:?}", t)).collect::<String>(),
                self.previous_hash,
                self.nonce
            );
            hash_data(data.as_bytes())
        }

        pub fn mine_block(&mut self, difficulty: u32) {
            let target = "0".repeat(difficulty as usize);
            while !self.hash.starts_with(&target) {
                self.nonce += 1;
                self.hash = self.calculate_hash();
            }
            println!("Block mined: {}", self.hash);
        }
    }


    #[derive(Debug)]
    pub struct Ledger {
        pub blockchain: Vec<Block>,
        pub difficulty: u32,
    }

    impl Ledger {
        pub fn new() -> Self {
            let genesis_block = Block::new(Utc::now().timestamp(), vec![], "0".to_string());
            Ledger {
                blockchain: vec![genesis_block],
                difficulty: 4,
            }
        }

        pub fn add_transaction(&mut self, transaction: Transaction) {
            // Verify the transaction signature before adding it to the block
            let data_hash = hash_data(&transaction.data);
            if !verify_signature(&data_hash, &transaction.signature, &transaction.public_key) {
                println!("Invalid transaction signature. Transaction rejected.");
                return;
            }

            let latest_block = self.blockchain.last().unwrap();
            let mut new_block = Block::new(Utc::now().timestamp(), vec![transaction], latest_block.hash.clone());
            new_block.mine_block(self.difficulty);
            self.blockchain.push(new_block);
        }

        pub fn mine_block(&mut self) {
            let latest_block = self.blockchain.last().unwrap();
            let mut new_block = Block::new(Utc::now().timestamp(), vec![], latest_block.hash.clone());
            new_block.mine_block(self.difficulty);
            self.blockchain.push(new_block);
        }

        pub fn is_chain_valid(&self) -> bool {
            for i in 1..self.blockchain.len() {
                let current_block = &self.blockchain[i];
                let previous_block = &self.blockchain[i - 1];

                if current_block.hash != current_block.calculate_hash() {
                    return false;
                }

                if current_block.previous_hash != previous_block.hash {
                    return false;
                }

                // Verify the transactions in the block
                for transaction in &current_block.transactions {
                    let data_hash = hash_data(&transaction.data);
                    if !verify_signature(&data_hash, &transaction.signature, &transaction.public_key) {
                        println!("Invalid transaction signature in block. Chain invalid.");
                        return false;
                    }
                }
            }
            true
        }
    }
}

pub mod crypto {
    use sha2::{Sha256, Digest};
    use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa::