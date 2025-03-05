# RustChain

# RustChain

RustChain is a simple blockchain implementation written in Rust. It includes basic blockchain functionality such as block mining with proof-of-work, transaction management, and a REST API for interacting with the blockchain.

## Features

- Proof-of-work blockchain with configurable mining difficulty
- Transaction pool for pending transactions
- REST API for blockchain interaction and queries
- Automatic mining in the background
- Basic peer discovery (WIP)
- Simple networking layer for node communication (WIP)

## Architecture

RustChain consists of several core components:

- **Blockchain**: Manages the chain of blocks and provides methods for adding and validating blocks
- **Block**: Represents a single block in the blockchain, including mining capabilities
- **Transaction**: Represents a transfer of value between parties
- **TransactionPool**: Manages pending transactions waiting to be included in blocks
- **API**: REST endpoints for interacting with the blockchain
- **Network**: Basic P2P communication (in progress)
- **Discovery**: Peer discovery mechanism (in progress)

## API Endpoints

OBS: You can use the request.json file to configure the API endpoints.

The following REST endpoints are available:

- `GET /blockchain` - Get the entire blockchain
- `GET /blockchain/stats` - Get statistics about the blockchain
- `GET /blockchain/validate` - Validate the integrity of the blockchain
- `GET /block/{index}` - Get a specific block by index
- `GET /block/latest` - Get the most recent block
- `POST /transaction` - Add a new transaction to the pool
- `POST /mine` - Mine a new block with transactions from the pool

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rustchain.git
   cd rustchain
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the node:
   ```
   cargo run --release
   ```

This will start a blockchain node on port 8080.

## Usage Examples

### Adding a transaction

```bash
curl -X POST http://localhost:8080/transaction \
  -H "Content-Type: application/json" \
  -d '{"sender":"alice","recipient":"bob","amount":5.0,"timestamp":1620000000}'
```

### Manually mine a block

```bash
curl -X POST http://localhost:8080/mine?num_transactions=5
```

### Get blockchain statistics

```bash
curl http://localhost:8080/blockchain/stats
```

### View the latest block

```bash
curl http://localhost:8080/block/latest
```

## Technical Details

- The blockchain uses SHA-256 for hashing blocks
- Proof-of-work difficulty is currently set to 4 (requires hashes to begin with 4 zeros)
- Automatic mining occurs every 15 seconds if transactions are available
- The genesis block is created automatically when the blockchain is initialized

## Future Enhancements

- Complete peer-to-peer networking functionality
- Implement UTXO-based transaction model
- Add cryptographic signatures for transactions
- Implement Merkle trees for efficient transaction verification
- Add wallet functionality
