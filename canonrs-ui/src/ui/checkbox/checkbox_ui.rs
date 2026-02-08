use leptos::prelude::*;
use crate::primitives::{
    CheckboxPrimitive,
    CheckboxIndicatorPrimitive,
};

#[component]
pub fn Checkbox(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2">
            <CheckboxPrimitive
                checked=checked
                disabled=disabled
                name=name
                value=value
                class=class
                id=id
            >
                <CheckboxIndicator />
            </CheckboxPrimitive>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CheckboxIndicator(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CheckboxIndicatorPrimitive class=class>
            "✓"
            {children.map(|c| c())}
        </CheckboxIndicatorPrimitive>
    }
}
