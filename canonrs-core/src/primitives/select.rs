//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA

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
            data-rs-state="closed"
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
            type="button"
            data-rs-select-trigger=""
            aria-haspopup="listbox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-controls={if controls_id.is_empty() { None } else { Some(controls_id) }}
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
            data-rs-state={if expanded { "open" } else { "closed" }}
            data-value-text=value_text
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
            data-placeholder=placeholder
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
            data-rs-select-content=""
            data-rs-state={if open { "open" } else { "closed" }}
            role="listbox"
            hidden={if !open { Some(true) } else { None }}
            id={if content_id.is_empty() { None } else { Some(content_id) }}
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SelectItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
    #[prop(default = -1_i32)] tabindex: i32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-select-item=""
            data-rs-state={if selected { "selected" } else { "unselected" }}
            data-value=move || value.get()
            role="option"
            tabindex=tabindex
            aria-selected={if selected { "true" } else { "false" }}
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
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
