use leptos::prelude::*;
use canonrs_core::primitives::TextareaPrimitive;
use canonrs_core::meta::DisabledState;

#[component]
pub fn Textarea(
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
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
            aria_labelledby=aria_labelledby.unwrap_or_default()
            aria_describedby=aria_describedby.unwrap_or_default()
            rows=rows.unwrap_or(3)
            class=class
        />
    }
}

#[component]
pub fn TextareaPreview() -> impl IntoView {
    view! { <Textarea placeholder="Type here..." /> }
}
