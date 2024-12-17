use consensus_app::server::ServerBuilder;
use tendermint_abci::KeyValueStoreApp;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let host = "127.0.0.1";
    let port = "26658";
    let read_buf_size = 1048576;

    let (app, driver) = KeyValueStoreApp::new();
    let server = ServerBuilder::new(read_buf_size)
        .bind(format!("{}:{}", host, port), app)
        .await
        .unwrap();

    std::thread::spawn(move || driver.run());
    server.listen().await.unwrap();
}
