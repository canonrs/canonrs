//! @canon-level: strict
//! @canon-owner: primitives-team
//! DocProgress Primitive
//!
//! Dois modos:
//!   1. Standalone — renderiza a barra com posicionamento próprio
//!   2. Portal — renderiza um portal anchor; o behavior injeta a barra no slot
//!      declarado pelo site via [data-rs-doc-progress-portal]

use leptos::prelude::*;

#[component]
pub fn DocProgressPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 0u8)] progress: u8,
) -> impl IntoView {
    let progress_str = progress.to_string();
    view! {
        <div
            data-rs-doc-progress=""
            data-rs-component="DocProgress"
            data-rs-behavior="feedback"
            data-rs-progress=progress_str.clone()
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=progress_str
            aria-label="Reading progress"
            class=class
        >
            <div data-rs-doc-progress-bar="" />
        </div>
    }
}

/// Portal anchor — coloque onde quiser no layout
/// O behavior vai injetar a barra aqui
#[component]
pub fn DocProgressPortal(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-doc-progress-portal=""
            data-rs-component="DocProgressPortal"
            data-rs-behavior="feedback"
            aria-hidden="true"
            class=class
        >
            <div data-rs-doc-progress-bar="" />
        </div>
    }
}
