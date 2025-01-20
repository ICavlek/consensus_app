use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use tendermint_abci::Application;
use tendermint_proto::abci::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, RequestInfo,
    RequestInitChain, RequestQuery, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEndBlock, ResponseInfo, ResponseInitChain, ResponseQuery,
};
use tracing::info;

use crate::transaction::Transaction;

pub const MAX_VARINT_LENGTH: usize = 16;

type ContractHash = String;
type Address = String;

#[derive(Clone)]
pub struct BlockchainApp {
    height: Cell<i64>,
    app_hash: Vec<u8>,
    storage: RefCell<HashMap<ContractHash, HashMap<Address, String>>>,
}

impl BlockchainApp {
    pub fn new() -> Self {
        Self {
            height: Cell::new(0),
            app_hash: vec![0_u8; MAX_VARINT_LENGTH],
            storage: RefCell::new(HashMap::new()),
        }
    }
}

impl Application for BlockchainApp {
    fn init_chain(&self, _request: RequestInitChain) -> ResponseInitChain {
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
            last_block_height: self.height.get(),
            last_block_app_hash: self.app_hash.clone().into(),
        }
    }

    fn query(&self, _request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let _tx: Vec<Transaction> = bincode::deserialize(&request.tx).unwrap();
        ResponseCheckTx {
            code: 0,
            ..Default::default()
        }
    }

    fn begin_block(&self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }

    fn deliver_tx(&self, request: RequestDeliverTx) -> ResponseDeliverTx {
        let tx: Vec<Transaction> = bincode::deserialize(&request.tx).unwrap();
        let height = self.height.get() + 1;
        self.height.set(height);
        let mut storage = self.storage.borrow_mut();
        storage.insert(tx[0].transaction_hash.clone(), HashMap::new());
        ResponseDeliverTx {
            code: 0,
            ..Default::default()
        }
    }

    fn end_block(&self, _request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    fn commit(&self) -> ResponseCommit {
        println!("COMMIT: {:#?}", self.storage);
        ResponseCommit {
            retain_height: 0,
            ..Default::default()
        }
    }
}
