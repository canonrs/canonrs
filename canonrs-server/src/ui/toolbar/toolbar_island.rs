//! @canon-level: strict
//! Toolbar Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::toolbar_ui::{Toolbar, ToolbarSeparator};
use canonrs_core::primitives::ToolbarOrientation;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolbarItem {
    pub label:           String,
    pub value:           String,
    pub disabled:        bool,
    pub separator_after: bool,
}



#[component]
pub fn ToolbarIsland(
    items: Vec<ToolbarItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] orientation: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let aria_label  = aria_label.unwrap_or_else(|| "Toolbar".to_string());
    let is_vertical = orientation.as_deref() == Some("vertical");
    let orient      = if is_vertical { ToolbarOrientation::Vertical } else { ToolbarOrientation::Horizontal };
    let class       = class.unwrap_or_default();

    let items_view = items.into_iter().map(|item| {
        let sep      = item.separator_after;
        let disabled = item.disabled;
        view! {
            <>
                <button
                    type="button"
                    data-rs-toolbar-item=""
                    data-rs-value=item.value
                    disabled=disabled
                    tabindex="-1"
                >
                    {item.label}
                </button>
                {if sep { Some(view! { <ToolbarSeparator /> }) } else { None }}
            </>
        }
    }).collect::<Vec<_>>();

    view! {
        <Toolbar aria_label=aria_label orientation=orient class=class>
            {items_view}
        </Toolbar>
    }
}
