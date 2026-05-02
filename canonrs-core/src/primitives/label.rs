//! @canon-level: strict
//! @canon-owner: primitives-team
//! Label Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn LabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let uid_lbl = crate::infra::uid::generate("lbl");
    view! {
        <label
            data-rs-label=""
            data-rs-uid=uid_lbl
            for={if html_for.is_empty() { None } else { Some(html_for) }}
            data-rs-required={if required { Some("") } else { None }}
            aria-required={if required { Some("true") } else { None }}
            class=class
        >
            {children()}
        </label>
    }
}
