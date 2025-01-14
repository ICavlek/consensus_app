use consensus_app::blockchain::BlockchainApp;
use consensus_app::server::ServerBuilder;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let host = "127.0.0.1";
    let port = "26658";
    let read_buf_size = 1048576;

    let app = BlockchainApp::new();
    let server = ServerBuilder::new(read_buf_size)
        .bind(format!("{}:{}", host, port), app)
        .await
        .unwrap();

    server.listen().await.unwrap();
}
