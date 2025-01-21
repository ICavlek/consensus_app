use std::{cell::RefCell, collections::HashMap};

use tendermint_abci::Application;
use tendermint_proto::abci::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, RequestInfo,
    RequestInitChain, RequestQuery, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEndBlock, ResponseInfo, ResponseInitChain, ResponseQuery,
};
use tracing::info;

use crate::transaction::{Transaction, TransactionType};

pub const MAX_VARINT_LENGTH: usize = 16;

type StateRoot = String;
type ContractHash = String;
type Address = String;
type ContractStore = HashMap<ContractHash, HashMap<Address, Contract>>;
type Block = (ContractStore, StateRoot);

#[derive(Clone, Debug)]
struct Contract {
    key: String,
    storage: String,
    proof: String,
}

#[derive(Clone)]
pub struct BlockchainApp {
    app_hash: Vec<u8>,
    blocks: RefCell<Vec<Block>>,
}

impl BlockchainApp {
    pub fn new() -> Self {
        Self {
            app_hash: vec![0_u8; MAX_VARINT_LENGTH],
            blocks: RefCell::new(vec![]),
        }
    }
}

impl Application for BlockchainApp {
    fn init_chain(&self, _request: RequestInitChain) -> ResponseInitChain {
        let mut blocks = self.blocks.borrow_mut();
        blocks.push((HashMap::new(), "0x0".to_string()));
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
            last_block_height: self.blocks.borrow().len() as i64,
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
        match tx.transaction_type {
            TransactionType::Declare { .. } => {
                let mut blocks = self.blocks.borrow_mut();
                let mut new_block = (HashMap::new(), "0x0".to_string());
                new_block.0.insert(tx.transaction_hash, HashMap::new());
                blocks.push(new_block);
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
        println!("COMMIT: {:#?}", self.blocks);
        ResponseCommit {
            retain_height: 0,
            ..Default::default()
        }
    }
}
