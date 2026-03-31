
use leptos::prelude::*;
use canonrs_core::primitives::doc_progress::{DocProgressPrimitive, DocProgressPortal};

/// Standalone — barra com posicionamento próprio (fixed top)
#[component]
pub fn DocProgress(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DocProgressPrimitive class=class progress=0u8 />
    }
}

/// Portal — coloque dentro do header ou onde quiser
/// O behavior injeta o scroll tracking aqui
#[component]
pub fn DocProgressSlot(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DocProgressPortal class=class />
    }
}
