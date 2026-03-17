use leptos::prelude::*;

#[component]
pub fn InputPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] data_variant: String,
    #[prop(default = String::new())] data_size: String,
) -> impl IntoView {
    view! {
        <input
            data-input=""
            data-variant={if data_variant.is_empty() { None } else { Some(data_variant) }}
            data-size={if data_size.is_empty() { None } else { Some(data_size) }}
            type={input_type}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
            name={if name.is_empty() { None } else { Some(name) }}
            prop:value=move || value.get()
            placeholder={if placeholder.is_empty() { None } else { Some(placeholder) }}
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            data-disabled=move || if disabled.get() { Some("true") } else { None }
            aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}
        />
    }
}
