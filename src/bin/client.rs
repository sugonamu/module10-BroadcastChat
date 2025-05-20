use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin_lines = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Read input from user
            result = stdin_lines.next_line() => {
                if let Ok(Some(line)) = result {
                    if !line.trim().is_empty() {
                        ws_stream.send(Message::text(line)).await?;
                    }
                }
            }

            // Receive message from server
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        let text = msg.as_text().unwrap();
                        println!("{text}");
                    }
                    Some(Ok(msg)) if msg.is_close() => break,
None => break,
                    _ => continue,
                }
            }
        }
    }

    Ok(())
}
