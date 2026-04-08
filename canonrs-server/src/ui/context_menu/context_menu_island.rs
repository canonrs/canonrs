//! ContextMenu Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/context_menu.rs

use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContextMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn ContextMenuIsland(
    children: Children,
    items: Vec<ContextMenuIslandItem>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::context_menu::init_all();
        });
    }

    let items_view = items.iter().map(|item| {
        let disabled = item.disabled;
        let label    = item.label.clone();
        view! {
            <div
                data-rs-context-menu-item=""
                role="menuitem"
                aria-disabled=disabled.to_string()
            >
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-context-menu=""
            data-rs-component="ContextMenu"
            data-rs-state="closed"
            class=class
        >
            <div data-rs-context-menu-trigger="">
                {children()}
            </div>
            <div
                data-rs-context-menu-content=""
                data-rs-state="closed"
                hidden=true
                role="menu"
            >
                {items_view}
            </div>
        </div>
    }
}
