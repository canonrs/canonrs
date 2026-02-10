use leptos::prelude::*;
use crate::primitives::{
    RadioGroupPrimitive,
    RadioGroupItemPrimitive,
    RadioGroupIndicatorPrimitive,
};

#[component]
pub fn RadioGroup(
    #[prop(into, optional)] id: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupPrimitive class={class} id={id.unwrap_or_default()}>
            {children.map(|c| c())}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into)] name: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let id_for_label = id.clone();
    let id_for_input = id.unwrap_or_default();
    
    view! {
        <label
            for={id_for_label}
            data-radio-item-wrapper=""
            attr:data-state={if checked { "checked" } else { "unchecked" }}
        >
            <RadioGroupItemPrimitive
                checked={checked}
                disabled={disabled}
                value={value}
                name={name}
                class={class}
                id={id_for_input}
            />
            <RadioGroupIndicator />
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn RadioGroupIndicator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupIndicatorPrimitive class={class}>
            "●"
        </RadioGroupIndicatorPrimitive>
    }
}
