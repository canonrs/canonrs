
use leptos::prelude::*;
use canonrs_core::primitives::InputGroupPrimitive;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] merge_radius: bool,
) -> impl IntoView {
    view! {
        <InputGroupPrimitive class=class merge_radius=merge_radius.into()>
            {children()}
        </InputGroupPrimitive>
    }
}

#[component]
pub fn InputGroupPreview() -> impl IntoView {
    view! {
        <InputGroup merge_radius=true>
            <span data-rs-input-group-addon="">"@"</span>
        </InputGroup>
    }
}
