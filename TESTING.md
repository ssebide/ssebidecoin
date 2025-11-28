# Ssebidecoin Testing Guide

This guide provides step-by-step instructions for testing the Ssebidecoin blockchain network.

## Prerequisites

Ensure you have the following wallet files (generate them using `cargo run -p lib --example keygen` if needed):
- `wallet_a.pub.pem` / `wallet_a.priv.cbor`
- `wallet_b.pub.pem` / `wallet_b.priv.cbor`
- `wallet_a_config.toml`
- `wallet_b_config.toml`

## Quick Start Test Scenario

### Step 1: Start the Node

Open **Terminal 1** and start the blockchain node:

```powershell
cargo run -p node
```

**Expected output:**
```
blockchain file exists, loading...
blockchain loaded
rebuilding utxos...
utxos rebuilt
initialization complete
Listening on 0.0.0.0:9000
```

> **Note:** If you see a port conflict error (os error 10048), stop any running node instances first.

**Keep this terminal running.**

---

### Step 2: Mine Initial Blocks

Open **Terminal 2** and start the miner to create coins for Wallet A:

```powershell
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

**Expected output:**
```
Mining block with target: ...
Submitting mined block
Block mined: <hash>
```

Let it mine **3-5 blocks** (each block generates 50 BTC mining reward), then press **Ctrl+C** to stop.

> **Tip:** Each block takes 15-60 seconds to mine depending on your CPU.

---

### Step 3: Check Wallet A Balance

In **Terminal 2**, check Wallet A's balance:

```powershell
cargo run -p wallet -- -c wallet_a_config.toml balance
```

**Expected output:**
```
💰 Balance: 150000000000 satoshis
   (1500 BTC)
```

The balance should show mining rewards from the blocks you mined (50 BTC per block × number of blocks).

---

### Step 4: Send Transaction to Wallet B

Send 1000 satoshis from Wallet A to Wallet B:

```powershell
cargo run -p wallet -- -c wallet_a_config.toml send --recipient WalletB --amount 1000
```

**Expected output:**
```
Fetching UTXOs...
Current balance: 150000000000 satoshis (1500 BTC)
Sending 1000 satoshis to WalletB...
✓ Transaction sent successfully
```

> **Note:** The transaction is now in the mempool but not yet confirmed.

---

### Step 5: Mine a Block to Confirm Transaction

Mine one more block to confirm the transaction:

```powershell
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

Wait for **one block** to be mined, then press **Ctrl+C**.

---

### Step 6: Verify Wallet B Received Funds

Check Wallet B's balance:

```powershell
cargo run -p wallet -- -c wallet_b_config.toml balance
```

**Expected output:**
```
💰 Balance: 1000 satoshis
   (0.00001 BTC)
```

✅ **Success!** Wallet B now has the 1000 satoshis sent from Wallet A.

---

## Advanced Testing Scenarios

### Test Multiple Transactions

1. Send multiple transactions from Wallet A to Wallet B:
   ```powershell
   cargo run -p wallet -- -c wallet_a_config.toml send --recipient WalletB --amount 5000
   cargo run -p wallet -- -c wallet_a_config.toml send --recipient WalletB --amount 10000
   ```

2. Mine a block to confirm all transactions

3. Check both wallet balances to verify the transfers

### Test Bidirectional Transfers

1. Send coins from Wallet B back to Wallet A:
   ```powershell
   cargo run -p wallet -- -c wallet_b_config.toml send --recipient WalletA --amount 500
   ```

2. Mine a block to confirm

3. Verify both balances updated correctly

### Test Insufficient Funds

Try sending more than the wallet balance:

```powershell
cargo run -p wallet -- -c wallet_b_config.toml send --recipient WalletA --amount 999999999
```

**Expected output:**
```
Error: Insufficient funds
```

---

## Troubleshooting

### Error: "Failed to deserialize Blockchain"

**Cause:** Race condition or corrupted blockchain file.

**Solution:** Delete `blockchain.cbor` and restart the node:
```powershell
Remove-Item blockchain.cbor
cargo run -p node
```

### Error: "Only one usage of each socket address... (os error 10048)"

**Cause:** Port 9000 is already in use by another node instance.

**Solution:** Stop the running node process or use a different port:
```powershell
cargo run -p node -- --port 9001
```

### Wallet shows 0 balance

**Cause:** No blocks have been mined with the wallet's public key as the reward address.

**Solution:** Mine blocks with the correct public key file:
```powershell
cargo run -p miner -- --address 127.0.0.1:9000 --public-key-file wallet_a.pub.pem
```

### Transaction not confirming

**Cause:** No new blocks have been mined to include the transaction.

**Solution:** Mine at least one block to confirm pending transactions.

### Connection refused

**Cause:** Node is not running or using a different address.

**Solution:** 
- Ensure the node is running on `127.0.0.1:9000`
- Check `default_node` in your wallet config files matches the node address

---

## Understanding the System

### Mining Rewards
- Each mined block generates **50 BTC** (5,000,000,000 satoshis) as a reward
- Rewards go to the address specified by `--public-key-file`
- Rewards halve every 210,000 blocks

### Transaction Lifecycle
1. **Create** - Wallet creates and signs transaction
2. **Broadcast** - Transaction sent to node's mempool
3. **Mine** - Miner includes transaction in new block
4. **Confirm** - Block added to blockchain, UTXOs updated

### UTXO Model
- Unspent Transaction Outputs (UTXOs) represent spendable coins
- Each transaction consumes UTXOs as inputs and creates new UTXOs as outputs
- Wallets scan the blockchain for UTXOs they can spend

---

## Clean Slate Testing

To start fresh with a new blockchain:

1. Stop all running nodes and miners
2. Delete the blockchain file:
   ```powershell
   Remove-Item blockchain.cbor
   ```
3. Start the node (it will create a new empty blockchain)
4. Mine new blocks to create coins

---

## Multi-Node Network Testing

### Start Second Node

In **Terminal 3**, start a second node on a different port:

```powershell
cargo run -p node -- --port 9001 127.0.0.1:9000
```

This connects the second node to the first node, creating a network.

### Test Network Propagation

1. Send a transaction through one node
2. Mine a block on the other node
3. Verify both nodes have the same blockchain state

---

Enjoy testing your cryptocurrency! 🚀