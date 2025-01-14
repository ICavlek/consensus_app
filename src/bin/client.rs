use tendermint_abci::ClientBuilder;
use tendermint_proto::abci::RequestInfo;

fn main() {
    let mut client = ClientBuilder::default().connect("127.0.0.1:26658").unwrap();
    let res = client
        .info(RequestInfo {
            version: "1".into(),
            block_version: 2,
            p2p_version: 3,
            abci_version: "4".into(),
        })
        .unwrap();
    println!("{}", res.last_block_height);
}
