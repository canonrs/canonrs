use leptos::prelude::*;

#[component]
pub fn ButtonPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
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
            disabled=move || disabled.get()
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            aria-label=aria_label
            data-rs-button=""
            data-ui-variant=data_variant
            data-ui-size=data_size
        >
            {children()}
        </button>
    }
}
