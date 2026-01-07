//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn ColorPickerPrimitive(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <input
            type="color"
            prop:value=move || value.get()
            on:input=move |ev| {
                value.set(event_target_value(&ev));
            }
            class=class
        />
    }
}
