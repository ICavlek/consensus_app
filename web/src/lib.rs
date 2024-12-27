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
        Verdict::Success => return JsValue::from_str("SUCCESS"),
        _ => return JsValue::from_str("FAILED"),
    }
}
