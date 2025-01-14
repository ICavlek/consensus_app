use std::sync::mpsc::{channel, Receiver, Sender};
use tendermint_abci::Application;
use tendermint_proto::abci::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, RequestInfo,
    RequestInitChain, RequestQuery, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEndBlock, ResponseInfo, ResponseInitChain, ResponseQuery,
};
use tracing::info;

pub const MAX_VARINT_LENGTH: usize = 16;

#[derive(Clone)]
pub struct BlockchainApp {
    cmd_tx: Sender<Command>,
}

impl BlockchainApp {
    pub fn new() -> (Self, BlockchainDriver) {
        let (cmd_tx, cmd_rx) = channel();
        (Self { cmd_tx }, BlockchainDriver::new(cmd_rx))
    }
}

impl Application for BlockchainApp {
    fn init_chain(&self, _request: RequestInitChain) -> ResponseInitChain {
        println!("INIT CHAIN");
        Default::default()
    }

    fn info(&self, request: RequestInfo) -> ResponseInfo {
        info!(
            "Got info request. Tendermint version: {}; Block version: {}; P2P version: {}",
            request.version, request.block_version, request.p2p_version
        );
        let (result_tx, result_rx) = channel();
        channel_send(&self.cmd_tx, Command::GetInfo { result_tx });
        let (last_block_height, last_block_app_hash) = channel_recv(&result_rx);

        ResponseInfo {
            data: "blockchain-rs".to_string(),
            version: "0.1.0".to_string(),
            app_version: 1,
            last_block_height,
            last_block_app_hash: last_block_app_hash.into(),
        }
    }

    fn query(&self, _request: RequestQuery) -> ResponseQuery {
        println!("QUERY");
        Default::default()
    }

    fn check_tx(&self, _request: RequestCheckTx) -> ResponseCheckTx {
        println!("CHECK TX");
        Default::default()
    }

    fn begin_block(&self, request: RequestBeginBlock) -> ResponseBeginBlock {
        println!("[BEGIN BLOCK] Hash: 0x{:x}", request.hash);
        Default::default()
    }

    fn deliver_tx(&self, request: RequestDeliverTx) -> ResponseDeliverTx {
        println!("[DELIVER TX] Bytes: 0x{:x}", request.tx);
        let tx = match std::str::from_utf8(&request.tx) {
            Ok(s) => s,
            Err(e) => panic!("Failed to interpret key as UTF-8: {e}"),
        };
        println!("[DELIVER TX] String: {}", tx);
        Default::default()
    }

    fn end_block(&self, _request: RequestEndBlock) -> ResponseEndBlock {
        println!("END BLOCK");
        Default::default()
    }

    fn commit(&self) -> ResponseCommit {
        let (result_tx, result_rx) = channel();
        channel_send(&self.cmd_tx, Command::Commit { result_tx });
        let (height, app_hash) = channel_recv(&result_rx);
        ResponseCommit {
            data: app_hash.into(),
            retain_height: height - 1,
        }
    }
}

pub struct BlockchainDriver {
    height: i64,
    app_hash: Vec<u8>,
    cmd_rx: Receiver<Command>,
}

impl BlockchainDriver {
    fn new(cmd_rx: Receiver<Command>) -> Self {
        Self {
            height: 0,
            app_hash: vec![0_u8; MAX_VARINT_LENGTH],
            cmd_rx,
        }
    }

    pub fn run(mut self) {
        loop {
            let cmd = self.cmd_rx.recv().unwrap();
            match cmd {
                Command::GetInfo { result_tx } => {
                    channel_send(&result_tx, (self.height, self.app_hash.clone()))
                }
                Command::Commit { result_tx } => {
                    self.commit(result_tx);
                }
            }
        }
    }

    fn commit(&mut self, result_tx: Sender<(i64, Vec<u8>)>) {
        self.height += 1;
        channel_send(&result_tx, (self.height, self.app_hash.clone()));
    }
}

enum Command {
    GetInfo { result_tx: Sender<(i64, Vec<u8>)> },
    Commit { result_tx: Sender<(i64, Vec<u8>)> },
}

fn channel_send<T>(tx: &Sender<T>, value: T) {
    tx.send(value).unwrap()
}

fn channel_recv<T>(rx: &Receiver<T>) -> T {
    rx.recv().unwrap()
}
