//! @canon-level: strict
//! ToggleGroup Island

use leptos::prelude::*;
use super::toggle_group_ui::ToggleGroup;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToggleGroupItem {
    pub value:    String,
    pub label:    String,
    pub on:       bool,
    pub disabled: bool,
}

static TOGGLE_GROUP_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);



#[island]
pub fn ToggleGroupIsland(
    items: Vec<ToggleGroupItem>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let multiple = multiple.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();
    let island_id = TOGGLE_GROUP_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();

    let items_view = items.into_iter().map(|item| {
        let base  = if item.on { "on" } else { "off" };
        let state = if item.disabled || disabled { format!("{} disabled", base) } else { base.to_string() };
        view! {
            <button
                type="button"
                data-rs-toggle=""
                data-rs-component="Toggle"
                data-rs-value=item.value
                data-rs-state=state
                aria-pressed=if item.on { "true" } else { "false" }
                role="button"
            >
                {item.label}
            </button>
        }
    }).collect::<Vec<_>>();

    view! {
        <ToggleGroup multiple=multiple class=class attr:data-rs-toggle-group-id=island_id>
            {items_view}
        </ToggleGroup>
    }
}
