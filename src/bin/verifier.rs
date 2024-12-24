use std::{str::FromStr, time::Duration};
use tendermint::{
    block::signed_header::SignedHeader, time::Time, trust_threshold::TrustThresholdFraction,
    validator::Info,
};
use tendermint_light_client_verifier::{
    options::Options,
    types::{LightBlock, PeerId, ValidatorSet},
    ProdVerifier, Verifier,
};

const ID: &str = "0bbdbdc5cd4da701d2158e892b36f6c611d5b5a6";

const SIGNED_HEADER_BEFORE: &str = r#"
    {
      "header": {
        "version": {
          "block": "11",
          "app": "1"
        },
        "chain_id": "test-chain-ZFfPjv",
        "height": "4",
        "time": "2024-12-24T09:45:54.887586Z",
        "last_block_id": {
          "hash": "5833B03570415F101385FD4B447691D7BEFCB13FA123E06FB91FB26F68069548",
          "parts": {
            "total": 1,
            "hash": "DB31139CA482CF10C153943F319A1902D1E50279E4DDD175378A5B7029B32B34"
          }
        },
        "last_commit_hash": "D8D57DDDC5725BE8234B3B0FE2A6E9C2B4260AAC3C9ECBE2FF209D1D532A741D",
        "data_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "validators_hash": "735B1309C5201307D5CCCEDAB296E77D2AD690DBB89EC2C909F1120D206DEBB0",
        "next_validators_hash": "735B1309C5201307D5CCCEDAB296E77D2AD690DBB89EC2C909F1120D206DEBB0",
        "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
        "app_hash": "00",
        "last_results_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "evidence_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "proposer_address": "45D4168CE8D4B54E04906F7BCDA4ECD9C8BA4707"
      },
      "commit": {
        "height": "4",
        "round": 0,
        "block_id": {
          "hash": "D5C99BDCD526AA8B8A32237AAEE86E44C8F7E89EE2C259DFCD8E5404CF5A12D4",
          "parts": {
            "total": 1,
            "hash": "0CA623EDDA978208CDCC9475100EDCD54CF20D18BF1DF1A9B6E196C602B8B771"
          }
        },
        "signatures": [
          {
            "block_id_flag": 2,
            "validator_address": "45D4168CE8D4B54E04906F7BCDA4ECD9C8BA4707",
            "timestamp": "2024-12-24T09:46:05.927492Z",
            "signature": "n1f8mHjHSTFSlF9vGg/+fxdW2tFSaB5TnWe1RlDk4FkjyWp+VUWXrtjGV4g8FomTwo9yrS5SnkzzYmd8Xge/BQ=="
          }
        ]
      }
    }"#;

const SIGNED_HEADER_AFTER: &str = r#"
    {
      "header": {
        "version": {
          "block": "11",
          "app": "1"
        },
        "chain_id": "test-chain-ZFfPjv",
        "height": "5",
        "time": "2024-12-24T09:46:05.927492Z",
        "last_block_id": {
          "hash": "D5C99BDCD526AA8B8A32237AAEE86E44C8F7E89EE2C259DFCD8E5404CF5A12D4",
          "parts": {
            "total": 1,
            "hash": "0CA623EDDA978208CDCC9475100EDCD54CF20D18BF1DF1A9B6E196C602B8B771"
          }
        },
        "last_commit_hash": "611C13521BA73E080F7E82DAB4989209DF106D552D3B90166B032E6D594616B2",
        "data_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "validators_hash": "735B1309C5201307D5CCCEDAB296E77D2AD690DBB89EC2C909F1120D206DEBB0",
        "next_validators_hash": "735B1309C5201307D5CCCEDAB296E77D2AD690DBB89EC2C909F1120D206DEBB0",
        "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
        "app_hash": "00",
        "last_results_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "evidence_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        "proposer_address": "45D4168CE8D4B54E04906F7BCDA4ECD9C8BA4707"
      },
      "commit": {
        "height": "5",
        "round": 0,
        "block_id": {
          "hash": "333E19898E1C68D4F6BDFF7DD29BD27B908A88CE464A10EEDD99732F63A2C213",
          "parts": {
            "total": 1,
            "hash": "13109329BE393D513C706D1D7A4ACC0F3246A59D7FE89C93A4597104CF2EA06E"
          }
        },
        "signatures": [
          {
            "block_id_flag": 2,
            "validator_address": "45D4168CE8D4B54E04906F7BCDA4ECD9C8BA4707",
            "timestamp": "2024-12-24T09:46:16.948386Z",
            "signature": "4PudTTCDUjVLS9qW/FqpdeL2RDWVIR1GSGFCV5+w/80DoljBYSxIQGJXG1NNPJ5Zg9utYHrLFtlvGPNbBUPOAg=="
          }
        ]
      }
    }"#;

const VALIDATOR: &str = r#"
    {
        "address": "45D4168CE8D4B54E04906F7BCDA4ECD9C8BA4707",
        "pub_key": {
          "type": "tendermint/PubKeyEd25519",
          "value": "UzNhsopdLDIOMaA6+WWRg2Q5zcFq0QKDICEfHGAlx7g="
        },
        "voting_power": "10",
        "proposer_priority": "0"
    }"#;

fn main() {
    let peer_id = PeerId::from_str(ID).unwrap();
    let header_before: SignedHeader = serde_json::from_str(SIGNED_HEADER_BEFORE).unwrap();
    let header_after: SignedHeader = serde_json::from_str(SIGNED_HEADER_AFTER).unwrap();
    let validator: Info = serde_json::from_str(VALIDATOR).unwrap();

    let validators = ValidatorSet::without_proposer(vec![validator]);

    let light_block_before = LightBlock::new(
        header_before,
        validators.clone(),
        validators.clone(),
        peer_id,
    );
    let light_block_after = LightBlock::new(header_after, validators.clone(), validators, peer_id);

    let options = Options {
        trust_threshold: TrustThresholdFraction::new(1, 3).unwrap(),
        trusting_period: Duration::new(1209600, 0),
        clock_drift: Duration::new(5, 0),
    };

    let verifier = ProdVerifier::default();
    let result= verifier.verify_update_header(
        light_block_after.as_untrusted_state(),
        light_block_before.as_trusted_state(),
        &options,
        Time::now(),
    );

    println!("{:#?}", result);
}
