
use leptos::prelude::*;
use canonrs_core::primitives::ToggleGroupPrimitive;

#[component]
pub fn ToggleGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    view! {
        <ToggleGroupPrimitive class=class multiple=multiple>
            {children()}
        </ToggleGroupPrimitive>
    }
}
