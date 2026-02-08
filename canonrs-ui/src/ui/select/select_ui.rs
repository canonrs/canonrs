use leptos::prelude::*;
use crate::primitives::{
    SelectPrimitive,
    SelectTriggerPrimitive,
    SelectValuePrimitive,
    SelectContentPrimitive,
    SelectItemPrimitive,
    SelectSeparatorPrimitive,
};

#[component]
pub fn Select(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SelectPrimitive
            class=class
            id=id
        >
            {children.map(|c| c())}
        </SelectPrimitive>
    }
}

#[component]
pub fn SelectTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] value_text: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SelectTriggerPrimitive
            disabled=disabled
            controls_id=controls_id
            expanded=expanded
            value_text=value_text
            class=class
            id=id
        >
            {children.map(|c| c())}
        </SelectTriggerPrimitive>
    }
}

#[component]
pub fn SelectValue(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectValuePrimitive
            placeholder=placeholder
            class=class
        >
            {children.map(|c| c())}
        </SelectValuePrimitive>
    }
}

#[component]
pub fn SelectContent(
    #[prop(optional)] children: Option<Children>,
    open: bool,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectContentPrimitive
            open=open
            content_id=content_id
            class=class
        >
            {children.map(|c| c())}
        </SelectContentPrimitive>
    }
}

#[component]
pub fn SelectItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SelectItemPrimitive
            tabindex={-1}
            selected=selected
            disabled=disabled
            value=value
            class=class
            id=id
        >
            {children.map(|c| c())}
        </SelectItemPrimitive>
    }
}

#[component]
pub fn SelectSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectSeparatorPrimitive
            class=class
        />
    }
}
