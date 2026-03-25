use leptos::prelude::*;

#[component]
pub fn RadioPrimitive(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label data-rs-radio-wrapper="" class={class}>
            <input
                type="radio"
                data-rs-radio=""
                prop:value=value
                name={name}
                checked={checked}
                disabled={disabled}
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-rs-radio-indicator=""></span>
            {children()}
        </label>
    }
}
