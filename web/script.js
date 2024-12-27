import init, { verify } from './pkg/web.js';

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
    let res = verify(untrusted_signed_header, trusted_signed_header, peer_id, validator);
    console.log(res);
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

await init();
connect_socket();
