use leptos::prelude::*;
use crate::primitives::{
    RadioGroupPrimitive,
    RadioGroupItemPrimitive,
    RadioGroupIndicatorPrimitive,
};

#[component]
pub fn RadioGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <RadioGroupPrimitive
            class=class
            id=id
        >
            {children.map(|c| c())}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <RadioGroupItemPrimitive
            tabindex={-1}
            checked=checked
            disabled=disabled
            value=value
            class=class
            id=id
        >
            {children.map(|c| c())}
        </RadioGroupItemPrimitive>
    }
}

#[component]
pub fn RadioGroupIndicator(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupIndicatorPrimitive
            class=class
        >
            {children.map(|c| c())}
        </RadioGroupIndicatorPrimitive>
    }
}
