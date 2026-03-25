//! @canon-level: ui
use leptos::prelude::*;
use canonrs_core::primitives::ToggleGroupPrimitive;

#[component]
pub fn ToggleGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    view! {
        <ToggleGroupPrimitive id=id class=class multiple=multiple>
            {children.map(|c| c())}
        </ToggleGroupPrimitive>
    }
}
