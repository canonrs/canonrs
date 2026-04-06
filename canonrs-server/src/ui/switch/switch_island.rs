use leptos::prelude::*;
use super::switch_ui::Switch;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

#[island]
pub fn SwitchIsland(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let initial  = checked.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let name     = name.unwrap_or_default();
    let value    = value.unwrap_or_default();
    let class    = class.unwrap_or_default();

    let root = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let el = match root.get() {
            Some(e) => e,
            None => return,
        };

        if el.has_attribute("data-rs-attached") { return; }
        el.set_attribute("data-rs-attached", "1").ok();

        // lê estado do DOM — não usa props
        let label = match el.query_selector("[data-rs-switch]").ok().flatten() {
            Some(e) => e,
            None => return,
        };

        let label_clone = label.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let state = label_clone.get_attribute("data-rs-state").unwrap_or_default();
            let next = if state.contains("selected") {
                state.replace("selected", "").trim().to_string()
            } else {
                format!("{} selected", state).trim().to_string()
            };
            label_clone.set_attribute("data-rs-state", &next).ok();
        }) as Box<dyn FnMut(_)>);

        label.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });

    view! {
        <div node_ref=root data-rs-switch-island="">
            <Switch
                checked=initial
                disabled=disabled
                name=name
                value=value
                class=class
            >
                <span></span>
            </Switch>
        </div>
    }
}
