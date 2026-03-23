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
            data-rs-select=""
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] value_text: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-rs-select-trigger=""
            type="button"
            role="button"
            tabindex="0"
            attr:aria-haspopup="listbox"
            attr:aria-controls={if controls_id.is_empty() { None } else { Some(controls_id) }}
            attr:aria-expanded={if expanded { "true" } else { "false" }}
            attr:aria-disabled={if disabled.get() { "true" } else { "false" }}
            data-rs-disabled={if disabled.get() { Some("true") } else { None }}
            attr:data-value-text={value_text}
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
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
            data-rs-select-value=""
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
            id={if content_id.is_empty() { None } else { Some(content_id) }}
            data-rs-select-content=""
            data-rs-state={if open { "open" } else { "closed" }}
            hidden={if !open { Some(true) } else { None }}
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
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-select-item=""
            attr:data-prop:value=move || value.get()
            role="option"
            tabindex={tabindex}
            attr:aria-selected={if selected { "true" } else { "false" }}
            attr:aria-disabled={if disabled.get() { "true" } else { "false" }}
            data-rs-state={if selected { "selected" } else { "unselected" }}
            data-rs-disabled={if disabled.get() { Some("true") } else { None }}
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
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
            data-rs-select-separator=""
            role="separator"
            class=class
        />
    }
}
