use leptos::prelude::*;
use canonrs_core::primitives::{SwitchPrimitive, SwitchThumbPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn Switch(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let checked_state  = if checked { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <SwitchPrimitive
            checked=checked_state
            disabled=disabled_state
            name=name
            value=value
            class=class
        >
            <SwitchThumbPrimitive />
            {children()}
        </SwitchPrimitive>
    }
}

#[component]
pub fn SwitchReactive(
    checked: RwSignal<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let on_click = move |_: leptos::ev::MouseEvent| {
        if !disabled { checked.update(|c| *c = !*c); }
    };

    view! {
        <label
            data-rs-switch=""
            data-rs-component="Switch"
            data-rs-checked=move || checked.get().to_string()
            data-rs-state=move || if checked.get() { "selected" } else { "" }
            class=class
            on:click=on_click
        >
            <input
                type="checkbox"
                data-rs-switch-input=""
                name=name
                value=value
                prop:checked=move || checked.get()
                disabled=disabled
                tabindex="-1"
                on:click=|e: leptos::ev::MouseEvent| e.stop_propagation()
            />
            <span data-rs-switch-thumb=""></span>
        </label>
    }
}

#[component]
pub fn SwitchPreview() -> impl IntoView {
    view! {
        <Switch>"Off"</Switch>
        <Switch checked=true>"On"</Switch>
        <Switch disabled=true>"Disabled"</Switch>
    }
}
