use leptos::prelude::*;
use canonrs_core::primitives::{CheckboxPrimitive, CheckboxIndicatorPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn Checkbox(
    children: Children,
    #[prop(default = ActivityState::Inactive)] checked: ActivityState,
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
