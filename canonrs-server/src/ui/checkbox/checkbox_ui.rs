//! @canon-level: ui
//! Checkbox - native HTML input, sem behavior
//! Estado gerenciado pelo browser via :checked/:disabled/:focus-visible

use leptos::prelude::*;
use canonrs_core::primitives::{
    CheckboxPrimitive,
    CheckboxIndicatorPrimitive,
};

#[component]
pub fn Checkbox(
    #[prop(into, default = String::new())] id: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let id_clone = id.clone();
    view! {
        <label for=id_clone data-rs-checkbox-wrapper="">
            <CheckboxPrimitive
                checked=checked
                disabled=disabled
                name=name
                class=class
                id=id
            />
            <CheckboxIndicatorPrimitive>
                "✓"
            </CheckboxIndicatorPrimitive>
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn CheckboxPreview() -> impl IntoView {
    view! {
        <Checkbox>"Remember me"</Checkbox>
    }
}
