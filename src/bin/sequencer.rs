use consensus_app::transaction::{Transaction, TransactionType};
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() {
    let tx_declare = Transaction::with_type(TransactionType::Declare {
        program: "./src/data/my_contract_hello.contract_class.json".to_string(),
    })
    .unwrap();
    let txs = vec![tx_declare];
    send_to_sequencer(txs).await;
}

async fn send_to_sequencer(txs: Vec<Transaction>) {
    // Check each transaction on tendermint.check call
    let tx = bincode::serialize(&txs).unwrap();

    let tendermint_client = HttpClient::new("http://127.0.0.1:26657").unwrap();
    let response = tendermint_client.broadcast_tx_sync(tx).await;
    // State root, from block, but I will store it in map = 0x06cbb5937c087bdece6ffe0a76300e097f082625ad407da1e62afb2f48bed6e7
    // Address = 0x0493429f345e634ae58eef2a3984540bdaaa37da0105636dd1d0e75898fe7cc0
    // Key = 0x0361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60
    // Result Storage = 0x000000000000000000000000000000000000000000000000000064696e616d6f
    println!("{:#?}", response);
}
