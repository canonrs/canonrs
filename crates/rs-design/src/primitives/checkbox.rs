//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxState {
    Checked,
    Unchecked,
    Indeterminate,
}

#[component]
pub fn CheckboxPrimitive(
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_checked_change: Callback<bool>,
    #[prop(optional)] class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let state = Memo::new(move |_| {
        if indeterminate.get() {
            CheckboxState::Indeterminate
        } else if checked.get() {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }
    });

    let handle_click = move |_| {
        if !disabled.get() {
            on_checked_change.run(!checked.get());
        }
    };

    view! {
        <button
            type="button"
            role="checkbox"
            aria-checked=move || match state.get() {
                CheckboxState::Checked => "true",
                CheckboxState::Unchecked => "false",
                CheckboxState::Indeterminate => "mixed",
            }
            aria-disabled=move || disabled.get()
            data-state=move || match state.get() {
                CheckboxState::Checked => "checked",
                CheckboxState::Unchecked => "unchecked",
                CheckboxState::Indeterminate => "indeterminate",
            }
            data-disabled=move || disabled.get()
            disabled=move || disabled.get()
            on:click=handle_click
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
