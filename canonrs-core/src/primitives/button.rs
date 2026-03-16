use leptos::prelude::*;

#[component]
pub fn ButtonPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = String::new())] data_variant: String,
    #[prop(default = String::new())] data_size: String,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            id=id
            class=class
            disabled=disabled
            aria-disabled=move || if disabled { "true" } else { "false" }
            aria-label=aria_label
            data-button=""
            data-ui-variant=data_variant
            data-ui-size=data_size
        >
            {children()}
        </button>
    }
}
