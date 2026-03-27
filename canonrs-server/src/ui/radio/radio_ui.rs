//! @canon-id: radio
//! @canon-label: Radio
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Select one option from a group
//! @canon-description: Radio button input
//! @canon-composable: true
//! @canon-capabilities: Disabled
//! @canon-required-parts: RadioGroup
//! @canon-optional-parts:
//! @canon-tags: radio, choice, exclusive, selection

use leptos::prelude::*;
use canonrs_core::primitives::RadioPrimitive;
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn Radio(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let selected = if checked { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <RadioPrimitive
            selected=selected
            disabled=disabled_state
            value=value
            name=name
            class=class
        >
            {children()}
        </RadioPrimitive>
    }
}

#[component]
pub fn RadioPreview() -> impl IntoView {
    view! {
        <Radio value="opt1" name="radio-preview">"Option 1"</Radio>
        <Radio value="opt2" name="radio-preview">"Option 2"</Radio>
    }
}
