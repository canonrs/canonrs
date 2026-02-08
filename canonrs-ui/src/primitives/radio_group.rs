//! @canon-level: strict
//! RadioGroup Primitive - Radio button group component

use leptos::prelude::*;

#[component]
pub fn RadioGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-radio-group=""
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
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-radio-group-item=""
            attr:data-value={value}
            type="button"
            role="radio"
            tabindex={tabindex}
            attr:aria-checked={if checked { "true" } else { "false" }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-state={if checked { "checked" } else { "unchecked" }}
            attr:data-disabled={if disabled { "true" } else { "" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn RadioGroupIndicatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            attr:data-radio-group-indicator=""
            class=class
        >
            {children.map(|c| c())}
        </span>
    }
}
