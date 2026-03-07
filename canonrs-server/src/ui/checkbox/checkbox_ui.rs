use leptos::prelude::*;
use canonrs_core::primitives::{
    CheckboxPrimitive,
    CheckboxIndicatorPrimitive,
};

#[component]
pub fn Checkbox(
    #[prop(into)] id: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let id_clone = id.clone();
    view! {
        <label for=id_clone data-checkbox-wrapper>
            <CheckboxPrimitive
                checked=checked
                disabled=disabled
                name=name
                value=value
                class=class
                id=id
            />
            <CheckboxIndicator />
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn CheckboxIndicator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CheckboxIndicatorPrimitive class=class>
            "✓"
        </CheckboxIndicatorPrimitive>
    }
}

#[component]
pub fn CheckboxPreview() -> impl IntoView {
    view! { <Checkbox id="cb-preview".to_string() /> }
}
