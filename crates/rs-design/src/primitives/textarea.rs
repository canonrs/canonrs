//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TextareaSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn TextareaPrimitive(
    #[prop(optional)] size: Option<TextareaSize>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] error: Option<bool>,
    #[prop(optional)] rows: Option<u32>,
) -> impl IntoView {
    let size = size.unwrap_or(TextareaSize::Md);
    let disabled = disabled.unwrap_or(false);
    let required = required.unwrap_or(false);
    let error = error.unwrap_or(false);
    let rows = rows.unwrap_or(4);

    view! {
        <textarea
            placeholder=placeholder
            disabled=disabled
            required=required
            rows=rows
            data-size=match size {
                TextareaSize::Sm => "sm",
                TextareaSize::Md => "md",
                TextareaSize::Lg => "lg",
            }
            data-error=error
        />
    }
}
