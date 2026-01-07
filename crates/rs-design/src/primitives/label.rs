//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn LabelPrimitive(
    #[prop(optional)] html_for: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    let disabled = disabled.unwrap_or(false);

    view! {
        <label
            r#for=html_for
            data-disabled=disabled
        >
            {children()}
        </label>
    }
}
