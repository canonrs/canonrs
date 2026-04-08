//! Drawer Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/drawer.rs

use leptos::prelude::*;

#[island]
pub fn DrawerIsland(
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] title:         Option<String>,
    #[prop(optional, into)] description:   Option<String>,
    #[prop(optional, into)] close_label:   Option<String>,
    #[prop(optional, into)] side:          Option<String>,
    #[prop(optional, into)] class:         Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Open".to_string());
    let close_label   = close_label.unwrap_or_else(|| "Close".to_string());
    let side          = side.unwrap_or_else(|| "left".to_string());

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::drawer::init_all();
        });
    }

    view! {
        <div
            data-rs-drawer=""
            data-rs-component="Drawer"
            data-rs-side=side
            data-rs-state="closed"
            class=class
        >
            <button
                type="button"
                data-rs-drawer-trigger=""
                data-rs-button=""
                data-rs-variant="primary"
                aria-haspopup="dialog"
                aria-expanded="false"
            >
                {trigger_label}
            </button>
            <div data-rs-drawer-portal="">
                <div data-rs-drawer-overlay=""></div>
                <div
                    data-rs-drawer-content=""
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    {title.map(|t| view! { <h2 data-rs-drawer-title="">{t}</h2> })}
                    {description.map(|d| view! { <p data-rs-drawer-description="">{d}</p> })}
                    <button
                        type="button"
                        data-rs-drawer-close=""
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
