//! DropdownMenu Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/dropdown_menu.rs

use leptos::prelude::*;
use super::dropdown_menu_ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
use canonrs_core::meta::DisabledState;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropdownMenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn DropdownMenuIsland(
    items: Vec<DropdownMenuIslandItem>,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class:         Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let trigger_label = trigger_label.unwrap_or_else(|| "Options".to_string());

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::dropdown_menu::init_all();
        });
    }

    let items_view = items.into_iter().map(|item| {
        let disabled = if item.disabled { DisabledState::Disabled } else { DisabledState::Enabled };
        view! {
            <DropdownMenuItem disabled=disabled>
                {item.label}
            </DropdownMenuItem>
        }
    }).collect::<Vec<_>>();

    view! {
        <DropdownMenu class=class>
            <DropdownMenuTrigger>{trigger_label}</DropdownMenuTrigger>
            <DropdownMenuContent>
                {items_view}
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
