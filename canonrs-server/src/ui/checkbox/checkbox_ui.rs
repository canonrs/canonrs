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
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let id_clone = id.clone();
    view! {
        <label for=id_clone data-rs-checkbox-wrapper="">
            <CheckboxPrimitive
                checked=checked
                disabled=disabled.get()
                name=name
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
    #[prop(into, default = String::new())] class: String,
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
