use leptos::prelude::*;
use canonrs_core::primitives::{CheckboxPrimitive, CheckboxIndicatorPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn Checkbox(
    children: Children,
    #[prop(default = ActivityState::Inactive)] checked: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label data-rs-checkbox-wrapper="">
            <CheckboxPrimitive
                checked=checked
                disabled=disabled
                name=name
                class=class
            />
            <CheckboxIndicatorPrimitive>
                "✓"
            </CheckboxIndicatorPrimitive>
            {children()}
        </label>
    }
}

#[component]
pub fn CheckboxReactive(
    checked: RwSignal<bool>,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let on_change = move |_: leptos::ev::Event| {
        if !disabled { checked.update(|c| *c = !*c); }
    };

    let focused = RwSignal::new(false);

    let state = move || {
        let mut s = vec![];
        if checked.get() { s.push("active"); }
        if focused.get() { s.push("focus"); }
        if disabled      { s.push("disabled"); }
        s.join(" ")
    };

    view! {
        <label
            data-rs-checkbox-wrapper=""
            class=class
        >
            <input
                type="checkbox"
                data-rs-checkbox-input=""
                data-rs-component="Checkbox"
                data-rs-state=state
                prop:checked=move || checked.get()
                disabled=disabled
                aria-checked=move || checked.get().to_string()
                name=if name.is_empty() { None } else { Some(name) }
                on:change=on_change
                on:focus=move |_| { focused.set(true); }
                on:blur=move  |_| { focused.set(false); }
            />
            <span data-rs-checkbox-indicator="" aria-hidden="true">
                "✓"
            </span>
            {children()}
        </label>
    }
}

#[component]
pub fn CheckboxPreview() -> impl IntoView {
    view! {
        <Checkbox>"Remember me"</Checkbox>
    }
}
