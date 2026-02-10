use leptos::prelude::*;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = false)] pressed: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-toggle=""
            attr:data-state={if pressed { "on" } else { "off" }}
            aria-pressed={if pressed { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            attr:aria-label={if aria_label.is_empty() { None } else { Some(aria_label.clone()) }}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </button>
    }
}
