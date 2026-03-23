use leptos::prelude::*;

#[component]
pub fn RadioPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label data-rs-radio-wrapper="" class={class}>
            <input
                type="radio"
                data-rs-radio=""
                prop:value=move || value.get()
                name={name}
                checked={checked}
                disabled={disabled}
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-rs-radio-indicator=""></span>
            {children.map(|c| c())}
        </label>
    }
}
