use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    println!("Connected to server! Type a message:");

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From server: {text}");
                        }
                    }
                    _ => break,
                }
            }
            line = stdin.next_line() => {
                if let Some(text) = line? {
                    ws_stream.send(Message::text(text)).await?;
                }
            }
        }
    }

    Ok(())
}