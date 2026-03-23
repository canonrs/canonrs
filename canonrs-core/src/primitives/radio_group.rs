use leptos::prelude::*;

#[component]
pub fn RadioGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
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
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn RadioGroupItemPrimitive(
    #[prop(optional)] _children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="radio"
            data-rs-radio-group-item=""
            prop:value=move || value.get()
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
        <span data-rs-radio-group-indicator="" class={class}>
            {children.map(|c| c())}
        </span>
    }
}
