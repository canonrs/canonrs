//! @canon-level: ui
//! ButtonGroup - sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::ButtonGroupPrimitive;

#[component]
pub fn ButtonGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] attached: bool,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonGroupPrimitive class=class attached=attached aria_label=aria_label.unwrap_or_default()>
            {children()}
        </ButtonGroupPrimitive>
    }
}

#[component]
pub fn ButtonGroupPreview() -> impl IntoView {
    view! {
        <ButtonGroup>
            <button type="button" data-rs-button="" data-ui-variant="outline">"Left"</button>
            <button type="button" data-rs-button="" data-ui-variant="outline">"Center"</button>
            <button type="button" data-rs-button="" data-ui-variant="outline">"Right"</button>
        </ButtonGroup>
    }
}
