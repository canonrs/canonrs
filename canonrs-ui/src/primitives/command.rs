//! @canon-level: strict
//! @canon-owner: primitives-team
//! Command Primitive - HTML puro para command palette

use leptos::prelude::*;

#[component]
pub fn CommandPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command=""
            role="application"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandInputPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let class_wrapper = class.clone();
    let id_wrapper = id.clone();
    
    view! {
        <div data-command-input-wrapper="" class={class_wrapper} id={id_wrapper}>
            <input
                attr:data-command-input=""
                type="text"
                placeholder={placeholder.unwrap_or_default()}
                value={value}
                class={class}
                id={id}
            />
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command-list=""
            role="listbox"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandEmptyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command-empty=""
            role="status"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] heading: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command-group=""
            role="group"
            class={class}
            id={id}
        >
            {heading.map(|h| view! {
                <div data-command-group-heading="">
                    {h}
                </div>
            })}
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command-item=""
            attr:data-value={value.unwrap_or_default()}
            attr:data-selected={if selected { "true" } else { "false" }}
            role="option"
            attr:aria-selected={if selected { "true" } else { "false" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-command-separator=""
            role="separator"
            attr:aria-orientation="horizontal"
            class={class}
            id={id}
        />
    }
}
