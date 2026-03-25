//! @canon-level: strict
//! Textarea Primitive - HTML textarea + ARIA

use leptos::prelude::*;

#[component]
pub fn TextareaPrimitive(
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] labelled_by: Option<String>,
    #[prop(optional)] described_by: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <textarea
            data-rs-textarea=""
            prop:value=value
            placeholder=placeholder
            name=name
            disabled=disabled
            aria-disabled={if disabled { Some("true") } else { None }}
            data-rs-disabled={if disabled { Some("true") } else { None }}
            readonly=readonly
            aria-readonly={if readonly { Some("true") } else { None }}
            data-rs-readonly={if readonly { Some("true") } else { None }}
            aria-required={if required { Some("true") } else { None }}
            data-rs-required={if required { Some("true") } else { None }}
            aria-labelledby=labelled_by
            aria-describedby=described_by
            rows=rows
            class=class
        />
    }
}
