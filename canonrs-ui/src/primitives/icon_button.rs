use leptos::prelude::*;

#[component]
pub fn IconButtonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-icon-button=""
            type="button"
            disabled={disabled}
            attr:aria-label={aria_label}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}
