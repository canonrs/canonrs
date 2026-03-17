use leptos::prelude::*;

#[component]
pub fn SwitchPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label data-switch="" class={class}>
            <input
                type="checkbox"
                checked={checked}
                disabled={disabled}
                name={if name.is_empty() { None } else { Some(name) }}
                prop:value=move || value.get()
                id={if id.is_empty() { None } else { Some(id) }}
            />
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn SwitchThumbPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-switch-thumb="" class={class} />
    }
}
