//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SwitchSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn SwitchPrimitive(
    #[prop(optional)] size: Option<SwitchSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] checked: Option<bool>,
) -> impl IntoView {
    let size = size.unwrap_or(SwitchSize::Md);
    let disabled = disabled.unwrap_or(false);
    let checked = checked.unwrap_or(false);

    view! {
        <button
            role="switch"
            aria-checked=checked
            tabindex=0
            disabled=disabled
            data-checked=checked
            data-size=match size {
                SwitchSize::Sm => "sm",
                SwitchSize::Md => "md",
                SwitchSize::Lg => "lg",
            }
        >
            <span data-thumb="true" />
        </button>
    }
}
