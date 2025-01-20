use anyhow::Result;
use serde::{Deserialize, Serialize};
use starknet::core::types::contract::SierraClass;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub transaction_hash: String,
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Declare {
        program: String,
    },
    DeployAccount {
        account: String,
    },
    Invoke {
        address: String,
        function: String,
        inputs: Option<Vec<i32>>,
    },
}

impl Transaction {
    pub fn with_type(transaction_type: TransactionType) -> Result<Transaction> {
        Ok(Transaction {
            transaction_hash: transaction_type.compute_and_hash()?,
            transaction_type,
            id: Uuid::new_v4().to_string(),
        })
    }
}

impl TransactionType {
    pub fn compute_and_hash(&self) -> Result<String> {
        match self {
            TransactionType::Declare { program } => {
                let contract = serde_json::from_reader::<_, SierraClass>(
                    std::fs::File::open(&program).unwrap(),
                )
                .unwrap();
                let class_hash = contract.class_hash().unwrap();
                Ok(format!("{class_hash:#064x}"))
            }
            TransactionType::DeployAccount { .. } => Ok("0x1".to_string()),
            TransactionType::Invoke {
                address,
                function,
                inputs,
            } => Ok(format!(
                "Invoked {function} with inputs {inputs:?} for contract in address {address}"
            )),
        }
    }
}
