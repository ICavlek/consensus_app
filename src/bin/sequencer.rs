use consensus_app::transaction::{Transaction, TransactionType};
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() {
    let tx = Transaction::with_type(TransactionType::Declare {
        program: "./src/data/my_contract_hello.contract_class.json".to_string(),
    })
    .unwrap();
    let tx = bincode::serialize(&tx).unwrap();

    let tendermint_client = HttpClient::new("http://127.0.0.1:26657").unwrap();
    let response = tendermint_client.broadcast_tx_async(tx).await;

    println!("{:#?}", response);
}
