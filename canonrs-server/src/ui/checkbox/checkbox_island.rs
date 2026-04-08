use leptos::prelude::*;
use super::checkbox_ui::Checkbox;
use canonrs_core::meta::{ActivityState, DisabledState};
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

fn add_state(el: &web_sys::Element, state: &str) {
    let mut states = el.get_attribute("data-rs-state").unwrap_or_default();
    if !states.split_whitespace().any(|s| s == state) {
        states = format!("{} {}", states, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &states).ok();
}

fn remove_state(el: &web_sys::Element, to_remove: &str) {
    let states = el.get_attribute("data-rs-state").unwrap_or_default();
    let filtered = states.split_whitespace()
        .filter(|s| *s != to_remove)
        .collect::<Vec<_>>()
        .join(" ");
    el.set_attribute("data-rs-state", &filtered).ok();
}

#[island]
pub fn CheckboxIsland(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let initial  = checked.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let name     = name.unwrap_or_default();
    let class    = class.unwrap_or_default();

    let root = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let el = match root.get() {
            Some(e) => e,
            None => return,
        };

        if el.has_attribute("data-rs-attached") { return; }
        el.set_attribute("data-rs-attached", "1").ok();

        // sincronizar estado inicial do DOM
        // change → toggle active no root
        {
            let el_c = el.clone();
            let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let is_checked = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .map(|i| i.checked())
                    .unwrap_or(false);
                if is_checked { add_state(&el_c, "active"); }
                else          { remove_state(&el_c, "active"); }
            }) as Box<dyn FnMut(_)>);
            el.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // focus → add focus no root
        {
            let el_c = el.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                add_state(&el_c, "focus");
            }) as Box<dyn FnMut(_)>);
            el.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // blur → remove focus no root
        {
            let el_c = el.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                remove_state(&el_c, "focus");
            }) as Box<dyn FnMut(_)>);
            el.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }
    });

    view! {
        <div
            node_ref=root
            data-rs-checkbox-island=""
            data-rs-state=if initial { "active" } else { "inactive" }
        >
            <Checkbox
                checked=if initial { ActivityState::Active } else { ActivityState::Inactive }
                disabled=if disabled { DisabledState::Disabled } else { DisabledState::Enabled }
                name=name
                class=class
            >
                {children()}
            </Checkbox>
        </div>
    }
}
