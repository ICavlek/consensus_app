use bitvec::prelude::{BitSlice, BitVec, Msb0};
use bitvec::view::BitView;
use eyre::{eyre, Result};
use iamgroot::jsonrpc;
use serde::{Deserialize, Serialize};
use starknet_crypto::{pedersen_hash, poseidon_hash_many, Felt as FieldElement};
use std::time::Duration;
use tendermint::{time::Time, trust_threshold::TrustThresholdFraction};
use tendermint_light_client_verifier::{
    options::Options,
    types::{LightBlock, ValidatorSet},
    ProdVerifier, Verdict, Verifier,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn verify(
    untrusted_signed_header: JsValue,
    trusted_signed_header: JsValue,
    peer_id: JsValue,
    validator: JsValue,
    now: JsValue,
) -> JsValue {
    let untrusted_signed_header = serde_wasm_bindgen::from_value(untrusted_signed_header).unwrap();
    let trusted_signed_header = serde_wasm_bindgen::from_value(trusted_signed_header).unwrap();
    let peer_id = serde_wasm_bindgen::from_value(peer_id).unwrap();
    let validator = serde_wasm_bindgen::from_value(validator).unwrap();
    let now: String = serde_wasm_bindgen::from_value(now).unwrap();

    let validators = ValidatorSet::without_proposer(vec![validator]);
    let trusted_light_block = LightBlock::new(
        trusted_signed_header,
        validators.clone(),
        validators.clone(),
        peer_id,
    );

    let untrusted_light_block = LightBlock::new(
        untrusted_signed_header,
        validators.clone(),
        validators.clone(),
        peer_id,
    );

    let verifier = ProdVerifier::default();
    let options = Options {
        trust_threshold: TrustThresholdFraction::new(1, 3).unwrap(),
        trusting_period: Duration::new(1209600, 0),
        clock_drift: Duration::new(5, 0),
    };

    let result = verifier.verify_update_header(
        untrusted_light_block.as_untrusted_state(),
        trusted_light_block.as_trusted_state(),
        &options,
        Time::parse_from_rfc3339(&now).unwrap(),
    );
    match result {
        Verdict::Success => return JsValue::from_str("SUCCESSFULLY VERIFIED BLOCK"),
        _ => return JsValue::from_str("FAILED TO VERIFY BLOCK"),
    }
}

#[wasm_bindgen]
pub fn prove(proof: JsValue) -> JsValue {
    let proof: GetProofResult = serde_wasm_bindgen::from_value(proof).unwrap();

    let state_root =
        Felt::try_new("0x391f30b5ba86364451d6e056c5d9427cc2204f99236a4b2a0f14ec237d11f90").unwrap();
    let contract_address = Address(
        Felt::try_new("0x493429f345e634ae58eef2a3984540bdaaa37da0105636dd1d0e75898fe7cc0").unwrap(),
    );
    let key =
        StorageKey::try_new("0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60")
            .unwrap();
    let result = Felt::try_new("0x64696e616d6f").unwrap();
    proof
        .verify(state_root, contract_address, key, result)
        .unwrap();

    JsValue::from_str("SUCCESSFULLY VERIFIED PROOF")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Felt(String);

mod felt {
    use super::jsonrpc;
    use super::Felt;
    use once_cell::sync::Lazy;
    use regex::Regex;

    static FELT_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$").expect("Felt: valid regex")
    });

    impl Felt {
        pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
            if FELT_REGEX.is_match(value) {
                Ok(Self(value.to_string()))
            } else {
                Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("Felt value does not match regex: {value}"),
                })
            }
        }
    }

    impl TryFrom<String> for Felt {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_new(&value).map_err(|e| e.message)
        }
    }

    impl AsRef<String> for Felt {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address(pub Felt);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Node {
    BinaryNode(BinaryNode),
    EdgeNode(EdgeNode),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryNode {
    pub binary: BinaryNodeBinary,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryNodeBinary {
    pub left: Felt,
    pub right: Felt,
}

type Proof = Vec<Node>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EdgeNode {
    pub edge: EdgeNodeEdge,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EdgeNodeEdge {
    pub child: Felt,
    pub path: EdgeNodePath,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EdgeNodePath {
    pub len: i64,
    pub value: Felt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractData {
    pub class_hash: Felt,
    pub contract_state_hash_version: Felt,
    pub nonce: Felt,
    pub root: Felt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_proofs: Option<Vec<Proof>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetProofResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub class_commitment: Option<Felt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contract_data: Option<ContractData>,
    pub contract_proof: Proof,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub state_commitment: Option<Felt>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct StorageKey(String);

mod storagekey {
    use super::jsonrpc;
    use super::StorageKey;
    use once_cell::sync::Lazy;
    use regex::Regex;

    static STORAGEKEY_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new("^0x(0|[0-7]{1}[a-fA-F0-9]{0,62}$)").expect("StorageKey: valid regex")
    });

    impl StorageKey {
        pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
            if STORAGEKEY_REGEX.is_match(value) {
                Ok(Self(value.to_string()))
            } else {
                Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("StorageKey value does not match regex: {value}"),
                })
            }
        }
    }

    impl TryFrom<String> for StorageKey {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_new(&value).map_err(|e| e.message)
        }
    }

    impl AsRef<String> for StorageKey {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl From<bool> for Direction {
    fn from(flag: bool) -> Self {
        if flag {
            Self::Right
        } else {
            Self::Left
        }
    }
}

impl GetProofResult {
    pub fn verify(
        &self,
        global_root: Felt,
        contract_address: Address,
        key: StorageKey,
        value: Felt,
    ) -> Result<(), jsonrpc::Error> {
        let contract_data = self.contract_data.as_ref().ok_or(jsonrpc::Error::new(
            -32700,
            "No contract data found".to_string(),
        ))?;
        self.verify_storage_proofs(contract_data, key, value)?;
        self.verify_contract_proof(contract_data, global_root, contract_address)
    }

    fn verify_storage_proofs(
        &self,
        contract_data: &ContractData,
        key: StorageKey,
        value: Felt,
    ) -> Result<(), jsonrpc::Error> {
        let root = &contract_data.root;
        let storage_proofs = &contract_data
            .storage_proofs
            .as_ref()
            .ok_or(jsonrpc::Error::new(
                -32700,
                "No storage proof found".to_string(),
            ))?[0];

        match Self::parse_proof(key.as_ref(), value, storage_proofs)? {
            Some(computed_root) if computed_root.as_ref() == root.as_ref() => Ok(()),
            Some(computed_root) => Err(jsonrpc::Error::new(
                -32700,
                format!(
                    "Proof invalid:\nprovided-root -> {}\ncomputed-root -> {}\n",
                    root.as_ref(),
                    computed_root.as_ref()
                ),
            )),
            None => Err(jsonrpc::Error::new(
                -32700,
                format!("Proof invalid for root -> {}\n", root.as_ref()),
            )),
        }
    }

    fn verify_contract_proof(
        &self,
        contract_data: &ContractData,
        global_root: Felt,
        contract_address: Address,
    ) -> Result<(), jsonrpc::Error> {
        let state_hash = Self::calculate_contract_state_hash(contract_data)?;

        match Self::parse_proof(
            contract_address.0.as_ref(),
            state_hash,
            &self.contract_proof,
        )? {
            Some(storage_commitment) => {
                let class_commitment = self.class_commitment.as_ref().ok_or(
                    jsonrpc::Error::new(-32700, "No class commitment".to_string()),
                )?;
                let parsed_global_root = Self::calculate_global_root(
                    class_commitment,
                    storage_commitment,
                )
                .map_err(|_| {
                    jsonrpc::Error::new(-32700, "Failed to calculate global root".to_string())
                })?;
                let state_commitment = self.state_commitment.as_ref().ok_or(
                    jsonrpc::Error::new(-32700, "No state commitment".to_string()),
                )?;
                if state_commitment.as_ref() == parsed_global_root.as_ref()
                    && global_root.as_ref() == parsed_global_root.as_ref()
                {
                    Ok(())
                } else {
                    Err(jsonrpc::Error::new(
                        -32700,
                        format!("Proof invalid:\nstate commitment -> {}\nparsed global root -> {}\n global root -> {}", 
                        state_commitment.as_ref(), parsed_global_root.as_ref(), global_root.as_ref())
                    ))
                }
            }
            None => Err(jsonrpc::Error::new(
                -32700,
                format!(
                    "Could not parse global root for root: {}",
                    global_root.as_ref()
                ),
            )),
        }
    }

    fn calculate_contract_state_hash(contract_data: &ContractData) -> Result<Felt, jsonrpc::Error> {
        // The contract state hash is defined as H(H(H(hash, root), nonce), CONTRACT_STATE_HASH_VERSION)
        const CONTRACT_STATE_HASH_VERSION: FieldElement = FieldElement::ZERO;
        let hash = pedersen_hash(
            &FieldElement::from_hex(contract_data.class_hash.as_ref()).map_err(|_| {
                jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
            })?,
            &FieldElement::from_hex(contract_data.root.as_ref()).map_err(|_| {
                jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
            })?,
        );
        let hash = pedersen_hash(
            &hash,
            &FieldElement::from_hex(contract_data.nonce.as_ref()).map_err(|_| {
                jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
            })?,
        );
        let hash = pedersen_hash(&hash, &CONTRACT_STATE_HASH_VERSION);
        Felt::try_new(&format!("0x{:x}", hash))
            .map_err(|_| jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string()))
    }

    fn calculate_global_root(
        class_commitment: &Felt,
        storage_commitment: Felt,
    ) -> Result<Felt, jsonrpc::Error> {
        let global_state_ver = FieldElement::from_bytes_be_slice(b"STARKNET_STATE_V0");
        let hash = poseidon_hash_many(&[
            global_state_ver,
            FieldElement::from_hex(storage_commitment.as_ref()).map_err(|_| {
                jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
            })?,
            FieldElement::from_hex(class_commitment.as_ref()).map_err(|_| {
                jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
            })?,
        ]);
        Felt::try_new(&format!("0x{:x}", hash))
            .map_err(|_| jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string()))
    }

    fn parse_proof(
        key: impl Into<String>,
        value: Felt,
        proof: &[Node],
    ) -> Result<Option<Felt>, jsonrpc::Error> {
        let key = FieldElement::from_hex(&key.into()).map_err(|_| {
            jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
        })?;
        let key = felt_to_bits(&key.to_bytes_be());
        if key.len() != 251 {
            return Ok(None);
        }
        let value = FieldElement::from_hex(value.as_ref()).map_err(|_| {
            jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
        })?;
        // initialized to the value so if the last node
        // in the proof is a binary node we can still verify
        let (mut hold, mut path_len) = (value, 0);
        // reverse the proof in order to hash from the leaf towards the root
        for (i, node) in proof.iter().rev().enumerate() {
            match node {
                Node::EdgeNode(EdgeNode {
                    edge: EdgeNodeEdge { child, path },
                }) => {
                    // calculate edge hash given by provider
                    let child_felt = FieldElement::from_hex(child.as_ref()).map_err(|_| {
                        jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
                    })?;
                    let path_value = FieldElement::from_hex(path.value.as_ref()).map_err(|_| {
                        jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
                    })?;
                    let provided_hash = pedersen_hash(&child_felt, &path_value)
                        + FieldElement::from(path.len as u64);
                    if i == 0 {
                        // mask storage key
                        let computed_hash =
                            match felt_from_bits(&key, Some(251 - path.len as usize)) {
                                Ok(masked_key) => {
                                    pedersen_hash(&value, &masked_key)
                                        + FieldElement::from(path.len as u64)
                                }
                                Err(_) => return Ok(None),
                            };
                        // verify computed hash against provided hash
                        if provided_hash != computed_hash {
                            return Ok(None);
                        };
                    }

                    // walk up the remaining path
                    path_len += path.len;
                    hold = provided_hash;
                }
                Node::BinaryNode(BinaryNode {
                    binary: BinaryNodeBinary { left, right },
                }) => {
                    path_len += 1;
                    let left = FieldElement::from_hex(left.as_ref()).map_err(|_| {
                        jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
                    })?;
                    let right = FieldElement::from_hex(right.as_ref()).map_err(|_| {
                        jsonrpc::Error::new(-32701, "Failed to create Field Element".to_string())
                    })?;
                    // identify path direction for this node
                    let expected_hash = match Direction::from(key[251 - path_len as usize]) {
                        Direction::Left => pedersen_hash(&hold, &right),
                        Direction::Right => pedersen_hash(&left, &hold),
                    };

                    hold = pedersen_hash(&left, &right);
                    // verify calculated hash vs provided hash for the node
                    if hold != expected_hash {
                        return Ok(None);
                    };
                }
            };
        }

        Ok(Some(Felt::try_new(&format!("0x{:x}", hold))?))
    }
}

fn felt_to_bits(felt: &[u8; 32]) -> BitVec<u8, Msb0> {
    felt.view_bits::<Msb0>()[5..].to_bitvec()
}

fn felt_from_bits(bits: &BitSlice<u8, Msb0>, mask: Option<usize>) -> Result<FieldElement> {
    if bits.len() != 251 {
        return Err(eyre!("expecting 251 bits"));
    }

    let mask = match mask {
        Some(x) => {
            if x > 251 {
                return Err(eyre!("Mask cannot be bigger than 251"));
            }
            x
        }
        None => 0,
    };

    let mut bytes = [0u8; 32];
    bytes.view_bits_mut::<Msb0>()[5 + mask..].copy_from_bitslice(&bits[mask..]);

    Ok(FieldElement::from_bytes_be(&bytes))
}
