use leptos::prelude::*;

#[component]
pub fn IconButtonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-icon-button=""
            disabled=move || disabled.get()
            aria-label=aria_label
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}
