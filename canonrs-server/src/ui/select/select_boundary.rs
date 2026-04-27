//! Select Island — Canon Rule passthrough
use leptos::prelude::*;

pub use canonrs_core::meta::{DisabledState, SelectionState};
pub use super::select_ui::{
    SelectTrigger, SelectValue, SelectContent, SelectItem, SelectSeparator,
};

#[component]
pub fn Select(
    children: Children,
    #[prop(optional)] node_ref: Option<leptos::prelude::NodeRef<leptos::html::Div>>,
    #[prop(default = canonrs_core::meta::DisabledState::Enabled)] disabled: canonrs_core::meta::DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <super::select_ui::Select class=class disabled=disabled node_ref=node_ref.unwrap_or_default()>
            {children()}
        </super::select_ui::Select>
    }
}
