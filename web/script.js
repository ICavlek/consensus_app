import init, { verify, prove } from './pkg/web.js';

var trusted_signed_header;
var peer_id;
var validator;

function connect_socket() {
  const ws = new WebSocket("ws://127.0.0.1:26657/websocket");
  const message = '{"jsonrpc": "2.0", "method": "subscribe", "id": 0, "params": {"query": "tm.event=\'NewBlock\'"}}';
  ws.onopen = (event) => {
    ws.send(message);
    trusted_signed_header = get_latest_signed_header();
    peer_id = get_peer_id();
    validator = get_validator();
  };
  ws.onmessage = (event) => {
    let untrusted_signed_header = get_latest_signed_header();
    let now = new Date().toISOString();
    let res = verify(untrusted_signed_header, trusted_signed_header, peer_id, validator, now);
    console.log(res);
    let proof = get_proof();
    let proof_value = prove(proof);
    console.log(proof_value);
  }
}

function get_peer_id() {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", "http://127.0.0.1:26657/status", false);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.send();
  let response = JSON.parse(xhr.responseText);
  return response.result.node_info.id;
}

function get_validator() {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", "http://127.0.0.1:26657/validators", false);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.send();
  let response = JSON.parse(xhr.responseText);
  return response.result.validators[0];
}

function get_latest_signed_header() {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", "http://127.0.0.1:26657/commit", false);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.send();
  let response = JSON.parse(xhr.responseText);
  return response.result.signed_header;
}

function get_proof() {
  let proof = JSON.parse('{"class_commitment":"0x700b2c54892319e6a0878ec3e2720753e0da8697963416f9f57bbf2f5d0fbb5","contract_data":{"class_hash":"0x6a367688d5c5a34af83593a5e9f0636ca6ad27ce5a44a9e471af5ea5e5e8a44","contract_state_hash_version":"0x0","nonce":"0x0","root":"0x3b5163e8e2d62ba3da127027979073ab4c56c8078511b355bd00bbd346f3525","storage_proofs":[[{"edge":{"child":"0x64696e616d6f","path":{"len":251,"value":"0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"}}}]]},"contract_proof":[{"binary":{"left":"0x2afa9f6d75e9a5c11abda05b7aee195f356e4c4150faa245f0866819b603961","right":"0x451ea2f4c703daa2280c15095b9a09a350e231712adf933228a9f82d6ef48ea"}},{"binary":{"left":"0x2eebb885385342add37257e451c45abebcb7911f018718f6a90b1acd452363c","right":"0x28a072ef2ef16f9cba087f8e13520b1bf7e21e7c43664c8e81ed8c85c375be"}},{"binary":{"left":"0x794a64bb23bcf72a272c3dc20c2dd6ed1bfc5b1aedb4d4e8e954442d2629347","right":"0x79015f3e1a5a818c150533a2ffb1efbfe8fcc07e57311e6626e1715f8bab697"}},{"binary":{"left":"0x2b06c6a7c5e5f1f434755ab8808c854e6f20d31aadb4829b700bee1f9261450","right":"0x507b64fefbf1b7cf4aa0a4beeadf6ea7fae2682d905521743f8fa63514c3f44"}},{"binary":{"left":"0x49bf7c6b17688ad7f2d163d314cfbcc20f4d366fd55737fc5f6f62ed81d6d8c","right":"0x1e0c1c17927ac6a1bf7130d12400757ac433036e07e677994e872a46e16069d"}},{"binary":{"left":"0x57db20687500842637a3d6fb33476ccd0b0f999b89d341f3fc9016e11898ed9","right":"0x697a4087d5836b43827948e949ed014b2e8b5bfb04b03001026f132a0d36381"}},{"binary":{"left":"0x6631759c4c0cf5c5beeedaf9878f9698ab58f56bec475264f084c826f3336e8","right":"0x5c0d033de8583781adb26aad989f6e9b29d92c791184db664a902557225b6a0"}},{"binary":{"left":"0x4f97c8aab7bf692a52521c8309dc0a7527ac96c850a5a8ffdb1009688ec998","right":"0x55af0ec87da52141a4c58a3d88985cd7028cf0f665005949d6684dabf7b5e4f"}},{"binary":{"left":"0x4b299827919097395dab6ccdf866497779f8477260e6792fcd47a182dfd65ba","right":"0x22468df9067ddd71ab88b981e4f05cd7565ec77d5ad4217ba6fdeca184a2f9b"}},{"binary":{"left":"0x77ad35554571199b5b58245d02f93312d8816887dc6c13688e3a2d504127435","right":"0x473c94dc5723cb4ef3ffbd71a536a39b977f86942054feab33d8e72d579f6e8"}},{"binary":{"left":"0x3fd1253dc24917b7901880bcf5cb82c14944a2c3c200b8be51be230a7d4c291","right":"0x3de2805b048d6c1bb06ae523f2ece570ec2cfa65ee71822b9a2b009d309300"}},{"binary":{"left":"0x605f0c03c4fdd9a0ff1e9f839089c75afa9d32320a32d063872c08d5a7d2cc2","right":"0x4bea65865884fa3b8ff8b051443509312847cad668d66d7fd76d9bb25b546e2"}},{"binary":{"left":"0x6ebd8c31d83108fdf6b69efd9125e77e5df0c83f582735b29c1e7ffb67b0f0b","right":"0x28099f6f90265960ffecdedd84c686b2ceb394275939f18aa00ca5ae2372d1c"}},{"binary":{"left":"0x66b9a7d6353280d187457c994cc82017ab73626f60f0fbce669c52a4c06ed4f","right":"0x1b1ef19db7c744d15ac30398737d3ce0158851b2710f41d5747111ba690cbe"}},{"binary":{"left":"0x7933cc047b768c1b23494bda18a3401d608c8d2da158b4d7e26ce544b60a0e0","right":"0x1baeae0108ae6e8211133dadb76ac57d49e81ccd5a6827cf046bc62893ff81b"}},{"binary":{"left":"0xc26e502f49fc7fa0f0a420a0513c4941a34f44f5400bbb0c19af51cd72dc59","right":"0x7d1036213f489baf7c0d831e5b22f4034905d764dff6f9733191f9802e7c0ed"}},{"binary":{"left":"0x26cec760304095968522e342819d2ffe3cf7dfc773c2a204e8f6603be250a93","right":"0x6a087ebb1c7b777bec9170d1b49081bbfcc23c4173fb39135039fc442b4a874"}},{"binary":{"left":"0x4cc9bb3e1beabee0a36f58d2069400bfcf54c4cf6df5c07ab78558ad2df92f","right":"0x412b36b70c0bf91b25641b12b3d4b06f2726f4c1421bfce3bc4241c38bba0ac"}},{"edge":{"child":"0xc3bbdbd75b244229ded425b3b2272991c2bd41572947a622b20d4cea51f478","path":{"len":233,"value":"0x9f345e634ae58eef2a3984540bdaaa37da0105636dd1d0e75898fe7cc0"}}}],"state_commitment":"0x391f30b5ba86364451d6e056c5d9427cc2204f99236a4b2a0f14ec237d11f90"}');
  return proof;
}

await init();
connect_socket();
