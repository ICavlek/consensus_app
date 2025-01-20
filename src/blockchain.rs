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

use crate::transaction::{Transaction, TransactionType};

pub const MAX_VARINT_LENGTH: usize = 16;

type ContractHash = String;
type Address = String;

#[derive(Clone, Debug)]
struct Contract {
    key: String,
    storage: String,
    proof: String,
}

#[derive(Clone)]
pub struct BlockchainApp {
    height: Cell<i64>,
    app_hash: Vec<u8>,
    contracts: RefCell<HashMap<ContractHash, HashMap<Address, Contract>>>,
    state_root: RefCell<String>,
}

impl BlockchainApp {
    pub fn new() -> Self {
        Self {
            height: Cell::new(0),
            app_hash: vec![0_u8; MAX_VARINT_LENGTH],
            contracts: RefCell::new(HashMap::new()),
            state_root: RefCell::new("0x0".to_string()),
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
        let txs: Vec<Transaction> = bincode::deserialize(&request.tx).unwrap();
        let tx = txs[0].clone();
        let height = self.height.get() + 1;
        self.height.set(height);
        match tx.transaction_type {
            TransactionType::Declare { .. } => {
                let mut contracts = self.contracts.borrow_mut();
                contracts.insert(tx.transaction_hash, HashMap::new());
            }
            TransactionType::Invoke { .. } => {}
            TransactionType::DeployAccount { .. } => {}
        }
        ResponseDeliverTx {
            code: 0,
            ..Default::default()
        }
    }

    fn end_block(&self, _request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    fn commit(&self) -> ResponseCommit {
        println!("COMMIT: {:#?}", self.contracts);
        ResponseCommit {
            retain_height: 0,
            ..Default::default()
        }
    }
}
