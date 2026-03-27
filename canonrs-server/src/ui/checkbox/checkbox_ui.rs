//! @canon-id: checkbox
//! @canon-label: Checkbox
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Toggle a boolean value
//! @canon-description: Checkbox input
//! @canon-composable: false
//! @canon-capabilities: Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: checkbox, check, tick, selection, multiple

use leptos::prelude::*;
use canonrs_core::primitives::{CheckboxPrimitive, CheckboxIndicatorPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn Checkbox(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let checked_state = if checked { ActivityState::Active } else { ActivityState::Inactive };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <label data-rs-checkbox-wrapper="">
            <CheckboxPrimitive
                checked=checked_state
                disabled=disabled_state
                name=name
                class=class
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
