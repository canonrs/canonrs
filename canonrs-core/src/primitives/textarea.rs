//! @canon-level: strict
//! Textarea Primitive - HTML textarea + ARIA

use leptos::prelude::*;

#[component]
pub fn TextareaPrimitive(
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = MaybeSignal::Static(false))] disabled: MaybeSignal<bool>,
    #[prop(into, default = MaybeSignal::Static(false))] readonly: MaybeSignal<bool>,
    #[prop(default = false)] required: bool,
    #[prop(optional)] labelled_by: Option<String>,
    #[prop(optional)] described_by: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <textarea
            attr:data-textarea=""
            prop:value={value}
            placeholder={placeholder}
            name={name}
            attr:aria-disabled=move || if disabled.get() { "true" } else { "false" }
            attr:data-disabled=move || if disabled.get() { Some("true") } else { None }
            attr:aria-readonly={if readonly.get() { "true" } else { "false" }}
            attr:data-readonly={if readonly.get() { Some("true") } else { None }}
            attr:aria-required={if required { "true" } else { "false" }}
            attr:data-required={if required { Some("true") } else { None }}
            attr:aria-labelledby={labelled_by}
            attr:aria-describedby={described_by}
            rows={rows}
            class=class
            id=id
        />
    }
}
