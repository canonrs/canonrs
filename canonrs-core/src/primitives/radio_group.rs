use leptos::prelude::*;

#[component]
pub fn RadioGroupPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-radio-group=""
            role="radiogroup"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn RadioGroupItemPrimitive(
    #[prop(optional)] _children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="radio"
            data-rs-radio-group-item=""
            prop:value=value
            name={name}
            checked={checked}
            disabled={disabled}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        />
    }
}

#[component]
pub fn RadioGroupIndicatorPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-radio-group-indicator="" class={class}>
            {children()}
        </span>
    }
}
