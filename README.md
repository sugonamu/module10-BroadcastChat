# Chat Async — Broadcast Chat Experiments

This project demonstrates a broadcast chat system using Rust's asynchronous programming model with `tokio` and `tokio-websockets`. It includes a WebSocket server and multiple clients communicating concurrently.

---

## Experiment 2.1: Original Code and How It Runs

- Created `server.rs` and `client.rs` under `src/bin/`.
- Implemented asynchronous WebSocket communication using `tokio-websockets`.
- Used `tokio::select!` in both server and client to concurrently:
  - Receive messages from the stream.
  - Broadcast/print messages.

### Screenshot:
![Experiment 2.1](./Experiment2.1.png)

### Observation:
Messages sent by one client are successfully received by all others. The server logs each client connection and forwards messages correctly.

---