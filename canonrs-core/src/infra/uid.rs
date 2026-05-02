//! UID — geração de identificadores únicos
//! CR-414: determinístico entre SSR e hydrate via Leptos context
//! O UidRoot provê um contador por árvore de renderização
//! Mesmo ordem SSR e WASM → mesmo UID

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use leptos::prelude::*;

/// Contexto de UID — um contador por árvore Leptos
#[derive(Clone)]
pub struct UidContext(Arc<AtomicU64>);

/// Componente raiz que provê o contexto de UID
/// Deve ser usado uma vez no topo da árvore (ex: CanonRSRoot)
#[component]
pub fn UidRoot(children: Children) -> impl IntoView {
    provide_context(UidContext(Arc::new(AtomicU64::new(0))));
    children()
}

/// Gera UID determinístico via contexto da árvore
/// Se não houver contexto (ex: testes), usa fallback global
pub fn generate(prefix: &str) -> String {
    let ctr = if let Some(ctx) = use_context::<UidContext>() {
        ctx.0.fetch_add(1, Ordering::SeqCst)
    } else {
        static FALLBACK: AtomicU64 = AtomicU64::new(0);
        FALLBACK.fetch_add(1, Ordering::SeqCst)
    };
    format!("{}-{:08x}", prefix, ctr)
}
