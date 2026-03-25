//! @canon-level: ui
//! RadioGroup - native HTML input, sem behavior
//! name compartilhado via prop no RadioGroupItem

use leptos::prelude::*;
use canonrs_core::primitives::{
    RadioGroupPrimitive,
    RadioGroupItemPrimitive,
    RadioGroupIndicatorPrimitive,
};

#[component]
pub fn RadioGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <RadioGroupPrimitive class={class} id={id.unwrap_or_default()}>
            {children()}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-radio-item-wrapper=""
            data-rs-state={if checked { "checked" } else { "unchecked" }}
        >
            <RadioGroupItemPrimitive
                checked={checked}
                disabled={disabled}
                value={value}
                name={name}
                class={class}
            />
            <RadioGroupIndicatorPrimitive>
                "●"
            </RadioGroupIndicatorPrimitive>
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn RadioGroupPreview() -> impl IntoView {
    view! {
        <RadioGroup>
            <RadioGroupItem name="rg-preview" value="a">"Option A"</RadioGroupItem>
            <RadioGroupItem name="rg-preview" value="b">"Option B"</RadioGroupItem>
        </RadioGroup>
    }
}
