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
    Declare { program: String },
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
                Ok(format!(
                    "{}{}",
                    "0x",
                    contract.class_hash().unwrap() // hex::encode(contract_hash.to_bytes_be())
                ))
            }
        }
    }
}
