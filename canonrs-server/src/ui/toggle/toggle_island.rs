use leptos::prelude::*;
use super::toggle_ui::Toggle;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

#[island]
pub fn ToggleIsland(
    #[prop(optional)] pressed: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let initial    = pressed.unwrap_or(false);
    let disabled   = disabled.unwrap_or(false);
    let aria_label = aria_label.unwrap_or_default();
    let class      = class.unwrap_or_default();

    let root = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let el = match root.get() {
            Some(e) => e,
            None => return,
        };

        if el.has_attribute("data-rs-js-attached") { return; }
        el.set_attribute("data-rs-js-attached", "1").ok();

        if disabled { return; }

        let el_clone = el.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let state = el_clone.get_attribute("data-rs-state").unwrap_or_default();
            let next  = if state.contains("on") { "off" } else { "on" };
            el_clone.set_attribute("data-rs-state", next).ok();
        }) as Box<dyn FnMut(_)>);

        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });

    let initial_state = if initial { "on" } else { "off" };

    view! {
        <div
            node_ref=root
            data-rs-toggle-island=""
            data-rs-state=initial_state
            class=class
        >
            <Toggle
                pressed=initial
                aria_label=aria_label
            >
                {children()}
            </Toggle>
        </div>
    }
}
