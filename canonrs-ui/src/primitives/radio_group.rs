//! @canon-level: strict
//! RadioGroup Primitive - Native HTML radio inputs

use leptos::prelude::*;

#[component]
pub fn RadioGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-radio-group="" style="display: flex; flex-direction: column; gap: var(--radio-group-gap);"
            role="radiogroup"
            class=class
            id=id
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
            value=value
            name=name
            checked=checked
            disabled=disabled
            class=class
            id=id
        />
    }
}

#[component]
pub fn RadioGroupIndicatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-radio-group-indicator=""
            class=class
        >
            {children.map(|c| c())}
        </span>
    }
}
