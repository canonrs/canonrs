//! CanonRS Dev Runtime — SSE reload route
//! Transparente para o app — injeta via with_dev_reload(router)
//! Ativo apenas em debug builds

#[cfg(feature = "ssr")]
pub mod reload {
    use axum::{Router, response::sse::{Event, Sse, KeepAlive}};
    use std::sync::Arc;
    use tokio::sync::broadcast;
    use futures_util::stream;

    /// Injeta a rota /canon-reload no router
    /// Uso: let app = canonrs::with_dev_reload(app);
    pub fn with_dev_reload(router: Router) -> Router {
        let (tx, _) = broadcast::channel::<()>(16);
        let tx = Arc::new(tx);

        // watcher do wasm_hash.js
        {
            let tx = tx.clone();
            let hash_path = std::env::var("CANON_ROOT")
                .map(|r| std::path::PathBuf::from(r)
                    .join("packages-rust/rs-canonrs/canonrs-client/assets/js/wasm_hash.js"))
                .unwrap_or_else(|_| std::path::PathBuf::from("canonrs-client/assets/js/wasm_hash.js"));

            tokio::spawn(async move {
                let mut last = String::new();
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    if let Ok(content) = tokio::fs::read_to_string(&hash_path).await {
                        if content != last {
                            last = content;
                            tx.send(()).ok();
                        }
                    }
                }
            });
        }

        router
        .route("/health", axum::routing::get(|| async { "ok" }))
        .route("/canon-reload", axum::routing::get({
            let tx = tx.clone();
            move || {
                let rx = tx.subscribe();
                async move {
                    let stream = stream::unfold(rx, |mut rx: broadcast::Receiver<()>| async move {
                        match rx.recv().await {
                            Ok(_) => Some((Ok::<Event, std::convert::Infallible>(Event::default().data("reload")), rx)),
                            Err(_) => None,
                        }
                    });
                    Sse::new(stream).keep_alive(KeepAlive::default())
                }
            }
        }))
    }
}
