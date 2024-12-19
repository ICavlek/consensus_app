import init, { add } from './pkg/web.js';

async function run() {
  await init();
  const result = add(1, 2);
  console.log(`1 + 2 = ${result}`);
  if (result !== 3) {
    throw new Error("wasm addition doesn't work!");
  }
}

function connect_socket() {
  const ws = new WebSocket("ws://127.0.0.1:26657/websocket");
  const message = '{"jsonrpc": "2.0", "method": "subscribe", "id": 0, "params": {"query": "tm.event=\'NewBlock\'"}}';
  ws.onopen = (event) => {
    ws.send(message);
  };
  ws.onmessage = (event) => {
    console.log(event.data);
  }
}

function send_status() {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", "http://127.0.0.1:26657/status", false);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.send();
  console.log(xhr.responseText);
}

run();
connect_socket();
send_status();
