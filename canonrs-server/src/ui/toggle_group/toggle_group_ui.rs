use leptos::prelude::*;
use canonrs_core::meta::DisabledState;
use canonrs_core::primitives::ToggleGroupPrimitive;

#[component]
pub fn ToggleGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] multiple: bool,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    view! {
        <ToggleGroupPrimitive class=class multiple=multiple disabled=disabled node_ref=node_ref.unwrap_or_default()>
            {children()}
        </ToggleGroupPrimitive>
    }
}
