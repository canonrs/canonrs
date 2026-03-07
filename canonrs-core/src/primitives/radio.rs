use leptos::prelude::*;

#[component]
pub fn RadioPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label data-radio-wrapper="" class={class}>
            <input
                type="radio"
                data-radio=""
                value={value}
                name={name}
                checked={checked}
                disabled={disabled}
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-radio-indicator=""></span>
            {children.map(|c| c())}
        </label>
    }
}
