use consensus_app::transaction::{Transaction, TransactionType};
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() {
    let tx_declare = Transaction::with_type(TransactionType::Declare {
        program: "./src/data/my_contract_hello.contract_class.json".to_string(),
    })
    .unwrap();
    send_to_sequencer(vec![tx_declare]).await;

    let tx_deploy = Transaction::with_type(TransactionType::Invoke {
        address: "0x0493429f345e634ae58eef2a3984540bdaaa37da0105636dd1d0e75898fe7cc0".to_string(),
        key: "0x0361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60".to_string(),
        storage: "0x64696e616d6f".to_string(),
    })
    .unwrap();
    send_to_sequencer(vec![tx_deploy]).await;
}

async fn send_to_sequencer(txs: Vec<Transaction>) {
    // Check each transaction on tendermint.check call
    let tx = bincode::serialize(&txs).unwrap();

    let tendermint_client = HttpClient::new("http://127.0.0.1:26657").unwrap();
    let response = tendermint_client.broadcast_tx_sync(tx).await;
    println!("{:#?}", response);
}
