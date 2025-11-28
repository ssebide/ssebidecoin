# Ssebidecoin

![Ssebidecoin Banner](assets/banner.png)

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**Ssebidecoin** is a simple cryptocurrency implementation in Rust. It follows the robust Bitcoin blockchain architecture, secure transaction handling using ECDSA and SHA256, and a peer-to-peer networking layer.

## 🚀 Features

-   **High Performance**: Built with Rust for speed and memory safety.
-   **Secure Cryptography**:
    -   **Elliptic Curve Digital Signature Algorithm (ECDSA)** for secure transaction signing.
    -   **SHA256** hashing for block and transaction integrity.
-   **UTXO Model**: Implements a robust Unspent Transaction Output model for tracking balances.
-   **Efficient Serialization**: Uses **CBOR** (Concise Binary Object Representation) for compact data storage and transmission.
-   **P2P Networking**: Custom peer-to-peer networking stack for decentralized node communication.
-   **Mining Support**: Proof-of-Work consensus mechanism with difficulty adjustment.

## 📦 Components

The project is organized into a workspace with the following crates:

-   **`lib`**: Core blockchain logic, data structures (Block, Transaction), and cryptography.
-   **`node`**: The full node implementation that maintains the blockchain and communicates with peers.
-   **`wallet`**: CLI wallet for managing keys, creating transactions, and checking balances.
-   **`miner`**: (Optional) Mining implementation to secure the network.

## 🛠️ Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/yourusername/ssebidecoin.git
    cd ssebidecoin
    ```

2.  **Build the project:**
    ```bash
    cargo build --release
    ```

## 💻 Usage

### Running a Node
Start a full node to join the network:
```bash
cargo run -p node
```

### Using the Wallet
Create a new wallet or send transactions:
```bash
# Generate a new wallet
cargo run -p wallet -- new

# Check balance
cargo run -p wallet -- balance --address <YOUR_ADDRESS>
```

### Mining
Start mining to earn rewards:
```bash
cargo run -p miner --address <YOUR_REWARD_ADDRESS>
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.
