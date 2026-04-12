//! Select Island — Canon Rule passthrough
use leptos::prelude::*;
use super::select_ui::{
    Select as SelectUi,
    SelectTrigger,
    SelectValue,
    SelectContent
};

#[component]
pub fn Select(
    children: Children,
    #[prop(optional)] node_ref: Option<leptos::prelude::NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::from("Select..."))] placeholder: String,
    #[prop(default = canonrs_core::meta::DisabledState::Enabled)] disabled: canonrs_core::meta::DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectUi class=class disabled=disabled node_ref=node_ref.unwrap_or_default()>
            <SelectTrigger>
                <SelectValue placeholder=placeholder>{""}</SelectValue>
            </SelectTrigger>
            <SelectContent>
                {children()}
            </SelectContent>
        </SelectUi>
    }
}
