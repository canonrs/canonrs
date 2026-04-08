//! @canon-level: strict
//! InputGroup Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;
use super::input_group_ui::InputGroup;
use canonrs_core::meta::ToggleState;

fn add_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace()
        .filter(|t| *t != token)
        .collect::<Vec<_>>()
        .join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

#[island]
pub fn InputGroupIsland(
    children: Children,
    #[prop(optional)] merge_radius: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let merge_radius = merge_radius.unwrap_or(false);
    let class        = class.unwrap_or_default();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_| {
                add_state(&root_cb, "focus-within");
            }));
            let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_| {
                remove_state(&root_cb, "focus-within");
            }));
            let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <InputGroup merge_radius=if merge_radius { ToggleState::On } else { ToggleState::Off } class=class node_ref=node_ref>
            {children()}
        </InputGroup>
    }
}
