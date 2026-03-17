//! @canon-level: strict
//! Textarea Primitive - HTML textarea + ARIA

use leptos::prelude::*;

#[component]
pub fn TextareaPrimitive(
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| false))] readonly: Signal<bool>,
    #[prop(default = false)] required: bool,
    #[prop(optional)] labelled_by: Option<String>,
    #[prop(optional)] described_by: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <textarea
            data-textarea=""
            prop:value=move || value.get()
            placeholder={placeholder}
            name={name}
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            data-disabled=move || if disabled.get() { Some("true") } else { None }
            aria-readonly=move || if readonly.get() { "true" } else { "false" }
            data-readonly=move || if readonly.get() { Some("true") } else { None }
            aria-required={if required { "true" } else { "false" }}
            data-required={if required { Some("true") } else { None }}
            aria-labelledby={labelled_by}
            aria-describedby={described_by}
            rows={rows}
            class=class
            id=id
        />
    }
}
