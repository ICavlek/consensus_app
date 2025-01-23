# Consensus Application
Server designed to communicate with tendermint, inspired by tendermint-rs example

## Overview
This project integrates a Starknet inspired sequencer, Tendermint consensus mechanism, and a Tendermint light client, providing a scalable solution for decentralized applications leveraging Tendermint's Byzantine Fault Tolerant (BFT) consensus connected with a simple blockchain storage.

## Components
### Starknet Sequencer
- Processes declare and invoke transaction
- Sends transaction to Tendermint
### Tendermint Proxy Application Server
- Processes transactions that come from Tendermint
- Executes Tendermint necessary function calls
- Creates simple blockchain
- Generates proof for transactions
### Tendermint Light Client
- Verifies the correctness of blocks without running a full node
- Verifies the proofs from the transactions

## Installation
### Prerequisites
- Go
- Rust
### Build & Run
1. Clone the repository
```bash
git clone https://github.com/your-username/starknet-tendermint.git
cd starknet-tendermint
```
2. Install dependencies
```bash
make install
```
3. Run the Proxy App Server
```bash
cargo run --bin server
```
4. Start the Tendermint node
```bash
tendermint init
tendermint start --consensus.create_empty_blocks_interval "10s"
```
5. Run the Light Client
```bash
cd web
wasm-pack build --target web
npx parcel build index.html
http-server dist
```
6. Run the Sequncer to declare contract
```bash
cargo run --bin sequencer -- declare
```
7. Run the Sequncer to deploy contract
```bash
cargo run --bin sequencer -- deploy
```

## Further Work
TODO

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments
- Starknet
- Tendermint
