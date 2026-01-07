//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn SelectPrimitive(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    value: RwSignal<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <select
            id=id
            name=name
            prop:value=move || value.get()
            on:change=move |ev| {
                let val = event_target_value(&ev);
                value.set(val.clone());
                if let Some(cb) = on_change {
                    cb.run(val);
                }
            }
            disabled=disabled
            required=required
            class=class
        >
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOptionPrimitive(
    #[prop(into)] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <option value=value disabled=disabled class=class>
            {children()}
        </option>
    }
}
