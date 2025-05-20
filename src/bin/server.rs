use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Incoming message from WebSocket client
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        let text = msg.as_text().unwrap().to_string();
                        let full_msg = format!("[{addr}]: {text}");
                        println!("{full_msg}");
                        let _ = bcast_tx.send(full_msg);
                    }
                    Some(Ok(msg)) if msg.is_close() => break,
                    None => break,
                    _ => continue,
                }
            }

            // Incoming broadcast message to send to this client
            result = bcast_rx.recv() => {
                match result {
                    Ok(msg) => {
                        ws_stream.send(Message::text(msg)).await?;
                    }
                    Err(_) => break,
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on ws://127.0.0.1:8000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");

        let bcast_tx = bcast_tx.clone();

        tokio::spawn(async move {
            match ServerBuilder::new().accept(socket).await {
                Ok((_req, ws_stream)) => {
                    if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                        eprintln!("Connection error: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket upgrade error: {e}");
                }
            }
        });
    }
}
