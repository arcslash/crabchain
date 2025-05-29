# 🦀 CrabChain

**CrabChain** is a minimal blockchain written in Rust.

---

## ✅ Features Implemented

- Custom blockchain with:
    - Block struct with hash, nonce, PoW mining, and timestamp
    - Hash chaining between blocks
    - Block validation logic
- Transaction model with sender, recipient, and amount
- Public-private key cryptography using Ed25519
    - Wallets with key generation
    - Message signing and verification
- `SignedTransaction` support with signature serialization
- CLI commands via [`clap`](https://docs.rs/clap)
- Balance tracking via chain analysis
- Mempool storage and JSON persistence

---

## 🛠️ Usage

Run via:

```bash
cargo run -- <COMMAND>
```

### 🔐 Wallet Management

```bash
# Generate a new wallet
cargo run -- wallet-new

# Get the wallet's public address
cargo run -- wallet-address --keyfile wallet.json
```

### 💸 Transactions

```bash
# Sign and prepare a transaction (adds to mempool)
cargo run -- send --keyfile wallet.json --from Alice --to Bob --amount 50
```

### ⛏️ Mining

```bash
# Mine a new block and include transactions
cargo run -- mine --miner Alice
```

### 💰 Balances

```bash
# Get final balances after mining
cargo run -- balances
```

---

## 📦 Dependencies

- `chrono` for timestamps
- `sha2` for block hashing
- `ed25519-dalek` for cryptographic signatures
- `serde` and `serde_json` for wallet/key serialization
- `clap` for command-line parsing

---