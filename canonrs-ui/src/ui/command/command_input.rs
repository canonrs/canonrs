use leptos::prelude::*;
use crate::primitives::command::CommandInputPrimitive;

#[component]
pub fn CommandInput(
    value: Signal<String>,
    #[prop(default = "Type a command...")] placeholder: &'static str,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CommandInputPrimitive
            attr:r#type="text"
            attr:placeholder={placeholder}
            attr:value={move || value.get()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
            attr:autofocus="true"
        />
    }
}
