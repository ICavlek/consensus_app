use tendermint_abci::ClientBuilder;
use tendermint_proto::abci::{RequestDeliverTx, RequestEcho, RequestQuery};

fn main() {
    let mut client = ClientBuilder::default().connect("127.0.0.1:26658").unwrap();
    let res = client
        .echo(RequestEcho {
            message: "DINAMOO".to_string(),
        })
        .unwrap();
    println!("{}", res.message);

    client
        .deliver_tx(RequestDeliverTx {
            tx: "test-key=test-value".into(),
        })
        .unwrap();
    client.commit().unwrap();

    let res = client
        .query(RequestQuery {
            data: "test-key".into(),
            path: "".to_string(),
            height: 0,
            prove: false,
        })
        .unwrap();
    println!("{:#?}", res.value);
}
