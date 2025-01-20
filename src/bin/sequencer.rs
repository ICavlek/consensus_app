use consensus_app::transaction::{Transaction, TransactionType};
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() {
    let tx_declare = Transaction::with_type(TransactionType::Declare {
        program: "./src/data/my_contract_hello.contract_class.json".to_string(),
    })
    .unwrap();
    let tx_deploy = Transaction::with_type(TransactionType::DeployAccount {
        account: "./src/data/account.json".to_string(),
    })
    .unwrap();
    let txs = vec![tx_declare, tx_deploy];
    send_to_sequencer(txs).await;
}

async fn send_to_sequencer(txs: Vec<Transaction>) {
    // Check each transaction on tendermint.check call
    let tx = bincode::serialize(&txs).unwrap();

    let tendermint_client = HttpClient::new("http://127.0.0.1:26657").unwrap();
    let response = tendermint_client.broadcast_tx_sync(tx).await;
    println!("{:#?}", response);
}
