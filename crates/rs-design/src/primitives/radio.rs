//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RadioSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn RadioPrimitive(
    #[prop(optional)] name: Option<String>,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] size: Option<RadioSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] checked: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or(RadioSize::Md);
    let disabled = disabled.unwrap_or(false);
    let checked = checked.unwrap_or(false);

    view! {
        <input
            type="radio"
            name=name
            value=value
            disabled=disabled
            checked=checked
            data-size=match size {
                RadioSize::Sm => "sm",
                RadioSize::Md => "md",
                RadioSize::Lg => "lg",
            }
        />
    }
}
