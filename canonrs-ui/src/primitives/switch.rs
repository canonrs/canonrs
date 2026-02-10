use leptos::prelude::*;

#[component]
pub fn SwitchPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-switch=""
            type="button"
            role="switch"
            tabindex="0"
            aria-checked={if checked { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            attr:aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}
            attr:data-checked={if checked { "true" } else { "false" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            attr:data-name={if name.is_empty() { None } else { Some(name) }}
            attr:data-value={if value.is_empty() { None } else { Some(value) }}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SwitchThumbPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <span data-switch-thumb="" class={class}>{children.map(|c| c())}</span>
    }
}
