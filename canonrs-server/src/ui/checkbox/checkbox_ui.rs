#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{CheckboxPrimitive, CheckboxIndicatorPrimitive};
use canonrs_core::primitives::CheckboxState;
use canonrs_core::meta::DisabledState;

#[component]
pub fn Checkbox(
    children: Children,
    #[prop(default = CheckboxState::Unchecked)] checked: CheckboxState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CheckboxPrimitive checked=checked disabled=disabled name=name class=class>
            <CheckboxIndicatorPrimitive>"✓"</CheckboxIndicatorPrimitive>
            {children()}
        </CheckboxPrimitive>
    }
}
