//! WebSocket reload server — porta 9099
//! Notifica browser quando WASM rebuilda

use tokio::sync::broadcast;
use crate::config::WS_PORT;

pub async fn ws_reload_server(reload_rx: broadcast::Receiver<()>) {
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use futures_util::{SinkExt, StreamExt};

    let addr = format!("0.0.0.0:{}", WS_PORT);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => { println!("[canon][ws] reload server on ws://localhost:{}", WS_PORT); l }
        Err(e) => { eprintln!("[canon][ws] FAILED to bind {}: {}", WS_PORT, e); return; }
    };

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => { eprintln!("[canon][ws] accept error: {}", e); continue; }
        };

        let mut rx = reload_rx.resubscribe();

        tokio::spawn(async move {
            let ws = match accept_async(stream).await {
                Ok(ws) => { println!("[canon][ws] client connected"); ws }
                Err(e) => { eprintln!("[canon][ws] handshake failed: {}", e); return; }
            };
            let (mut write, mut read) = ws.split();
            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(tokio_tungstenite::tungstenite::Message::Close(_))) => break,
                            Some(Ok(_)) => {}
                            Some(Err(_)) | None => break,
                        }
                    }
                    evt = rx.recv() => {
                        match evt {
                            Ok(_) => {
                                let _ = write.send(
                                    tokio_tungstenite::tungstenite::Message::Text("reload".into())
                                ).await;
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => break,
                        }
                    }
                }
            }
        });
    }
}
