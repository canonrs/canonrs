#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::TogglePrimitive;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] pressed: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <TogglePrimitive class=class pressed=pressed.into() disabled=disabled.into() aria_label=aria_label>
            {children()}
        </TogglePrimitive>
    }
}

#[component]
pub fn ToggleReactive(
    pressed: RwSignal<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let aria_label = aria_label.unwrap_or_default();
    let class      = class.unwrap_or_default();

    let on_click = move |_: leptos::ev::MouseEvent| {
        if !disabled { pressed.update(|p| *p = !*p); }
    };

    view! {
        <label
            data-rs-toggle=""
            data-rs-component="Toggle"
            data-rs-state=move || if pressed.get() { "on" } else { "off" }
            class=class
            aria-label=if aria_label.is_empty() { None } else { Some(aria_label) }
            on:click=on_click
        >
            <input
                type="checkbox"
                data-rs-toggle-input=""
                prop:checked=move || pressed.get()
                disabled=disabled
                tabindex="-1"
                on:click=|e: leptos::ev::MouseEvent| e.stop_propagation()
            />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}

#[component]
pub fn TogglePreview() -> impl IntoView {
    view! {
        <Toggle>"Toggle"</Toggle>
        <Toggle pressed=true>"Active"</Toggle>
    }
}
