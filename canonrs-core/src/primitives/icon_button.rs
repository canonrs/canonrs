use leptos::prelude::*;

#[component]
pub fn IconButtonPrimitive(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-icon-button=""
            disabled=disabled
            aria-label=aria_label
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </button>
    }
}
