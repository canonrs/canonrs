use leptos::prelude::*;

#[component]
pub fn RadioGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-radio-group=""
            role="radiogroup"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn RadioGroupItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="radio"
            data-radio-group-item=""
            value={value}
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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-radio-group-indicator="" class={class}>
            {children.map(|c| c())}
        </span>
    }
}
