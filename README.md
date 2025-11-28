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
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

### 🐳 Docker Support

You can also run Ssebidecoin using Docker:

```bash
# Build the image
docker build -t ssebidecoin .

# Run a node
docker run -d -p 9000:9000 -v ssebidecoin-data:/data ssebidecoin

# Run the wallet
docker run -it --rm --entrypoint wallet ssebidecoin --help
```

## 🧪 Testing

### Quick Start Test

Follow these steps to test the complete blockchain functionality:

#### 1. Start the Node
```bash
cargo run -p node
```

#### 2. Generate Wallet Keys (if needed)
```bash
cargo run -p lib --example keygen
```

This creates wallet key pairs and configuration files.

#### 3. Mine Initial Blocks
```bash
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

Let it mine 3-5 blocks (each generates 50 BTC reward), then press `Ctrl+C`.

#### 4. Check Wallet Balance
```bash
cargo run -p wallet -- -c wallet_a_config.toml balance
```

Expected output: `💰 Balance: 150000000000 satoshis (1500 BTC)`

#### 5. Send Transaction
```bash
cargo run -p wallet -- -c wallet_a_config.toml send --recipient WalletB --amount 1000
```

#### 6. Mine Block to Confirm
```bash
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

Wait for one block, then press `Ctrl+C`.

#### 7. Verify Transaction
```bash
cargo run -p wallet -- -c wallet_b_config.toml balance
```

Expected output: `💰 Balance: 1000 satoshis (0.00001 BTC)`

✅ **Success!** The transaction was confirmed on the blockchain.

### Full Testing Guide

For comprehensive testing instructions, troubleshooting, and advanced scenarios, see [TESTING.md](TESTING.md).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.
