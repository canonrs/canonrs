//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA
//! Base: Button + Listbox (NÃO combobox - sem input editável)

use leptos::prelude::*;

#[component]
pub fn SelectPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-select=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] value_text: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-select-trigger=""
            type="button"
            role="button"
            tabindex="0"
            attr:aria-haspopup="listbox"
            attr:aria-controls={controls_id}
            attr:aria-expanded={if expanded { "true" } else { "false" }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            attr:data-value-text={value_text}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SelectValuePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            attr:data-select-value=""
            attr:data-placeholder={placeholder}
            class=class
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn SelectContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: bool,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            role="listbox"
            id={content_id}
            attr:data-select-content=""
            attr:data-state={if open { "open" } else { "closed" }}
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-select-item=""
            attr:data-value={value}
            role="option"
            tabindex={tabindex}
            attr:aria-selected={if selected { "true" } else { "false" }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-state={if selected { "selected" } else { "unselected" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-select-separator=""
            role="separator"
            class=class
        />
    }
}
