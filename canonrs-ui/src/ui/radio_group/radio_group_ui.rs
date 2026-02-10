use leptos::prelude::*;
use crate::primitives::{
    RadioGroupPrimitive,
    RadioGroupItemPrimitive,
    RadioGroupIndicatorPrimitive,
};

#[component]
pub fn RadioGroup(
    #[prop(into)] id: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
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
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let id_clone = id.clone();
    view! {
        <label for=id_clone data-radio-item-wrapper style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer;">
            <RadioGroupItemPrimitive
                checked=checked
                disabled=disabled
                value=value
                name=name
                class=class
                id=id
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
        <RadioGroupIndicatorPrimitive class=class>
            "●"
        </RadioGroupIndicatorPrimitive>
    }
}
