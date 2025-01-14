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
    height: i64,
    app_hash: Vec<u8>,
}

impl BlockchainApp {
    pub fn new() -> Self {
        Self {
            height: 0,
            app_hash: vec![0_u8; MAX_VARINT_LENGTH],
        }
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

        ResponseInfo {
            data: "blockchain-rs".to_string(),
            version: "0.1.0".to_string(),
            app_version: 1,
            last_block_height: self.height,
            last_block_app_hash: self.app_hash.clone().into(),
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
        ResponseCommit {
            data: self.app_hash.clone().into(),
            retain_height: self.height - 1,
        }
    }
}
