use leptos::prelude::*;
use canonrs_core::primitives::TextareaPrimitive;

#[component]
pub fn Textarea(
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = MaybeSignal::Static(false))] disabled: MaybeSignal<bool>,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] labelled_by: Option<String>,
    #[prop(optional, into)] described_by: Option<String>,
    #[prop(optional, into)] rows: Option<u32>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TextareaPrimitive
            value={value}
            placeholder={placeholder}
            name={name}
            disabled=disabled
            readonly={readonly}
            required={required}
            labelled_by={labelled_by.unwrap_or_default()}
            described_by={described_by.unwrap_or_default()}
            rows={rows.unwrap_or(3)}
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn TextareaPreview() -> impl IntoView {
    view! { <Textarea placeholder="Type here...".to_string() /> }
}
