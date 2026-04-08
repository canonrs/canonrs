//! Modal Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/modal.rs

use leptos::prelude::*;

#[island]
pub fn ModalIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] description:   Option<String>,
    #[prop(optional, into)] close_label:   Option<String>,
    #[prop(optional, into)] class:         Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Open".to_string());
    let close_label   = close_label.unwrap_or_else(|| "Close".to_string());

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::modal::init_all();
        });
    }

    view! {
        <div
            data-rs-modal=""
            data-rs-component="Modal"
            data-rs-state="closed"
            class=class
        >
            <button
                type="button"
                data-rs-modal-trigger=""
                data-rs-button=""
                data-rs-variant="primary"
                aria-haspopup="dialog"
                aria-expanded="false"
            >
                {trigger_label}
            </button>
            <div data-rs-modal-portal="">
                <div data-rs-modal-overlay=""></div>
                <div
                    data-rs-modal-content=""
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    {description.map(|d| view! { <p data-rs-modal-description="">{d}</p> })}
                    <button
                        type="button"
                        data-rs-modal-close=""
                        data-rs-button=""
                        data-rs-variant="outline"
                    >
                        {close_label}
                    </button>
                </div>
            </div>
        </div>
    }
}
