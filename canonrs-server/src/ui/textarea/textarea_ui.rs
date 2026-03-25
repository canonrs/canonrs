//! @canon-level: ui
//! Textarea - sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::TextareaPrimitive;

#[component]
pub fn Textarea(
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
        <TextareaPrimitive
            value=value
            placeholder=placeholder
            name=name
            disabled=disabled
            readonly=readonly
            required=required
            labelled_by=labelled_by.unwrap_or_default()
            described_by=described_by.unwrap_or_default()
            rows=rows.unwrap_or(3)
            class=class
        />
    }
}

#[component]
pub fn TextareaPreview() -> impl IntoView {
    view! { <Textarea placeholder="Type here..." /> }
}
