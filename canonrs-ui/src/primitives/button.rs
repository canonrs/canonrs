use leptos::prelude::*;

#[component]
pub fn ButtonPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            id={id}
            class={class}
            disabled={disabled}
            attr:aria-disabled={if disabled { "true" } else { "" }}
            aria-label={aria_label}
        >
            {children.map(|c| c())}
        </button>
    }
}
