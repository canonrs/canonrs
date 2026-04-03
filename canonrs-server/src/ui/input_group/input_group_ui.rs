use leptos::prelude::*;
use canonrs_core::primitives::InputGroupPrimitive;
use canonrs_core::meta::ToggleState;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] merge_radius: ToggleState,
) -> impl IntoView {
    view! {
        <InputGroupPrimitive class=class merge_radius=merge_radius>
            {children()}
        </InputGroupPrimitive>
    }
}

#[component]
pub fn InputGroupPreview() -> impl IntoView {
    view! {
        <InputGroup merge_radius=ToggleState::On>
            <span data-rs-input-group-addon="">"@"</span>
        </InputGroup>
    }
}
