//! Tooltip Island — Canon Rule #341 + #342
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

#[island]
pub fn TooltipIsland(
    #[prop(into)] label: String,
    #[prop(into)] content: String,
    #[prop(optional, into)] side: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let side  = side.unwrap_or_else(|| "top".to_string());

    let node_ref = NodeRef::<leptos::html::Span>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        let trigger = match root.query_selector("[data-rs-tooltip-trigger]").ok().flatten() {
            Some(el) => el, None => return,
        };

        // mouseenter → open
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let _ = root_cb.set_attribute("data-rs-state", "open");
                if let Ok(Some(c)) = root_cb.query_selector("[data-rs-tooltip-content]") {
                    let _ = c.set_attribute("data-rs-state", "open");
                }
            }));
            let _ = trigger.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // mouseleave → closed
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let _ = root_cb.set_attribute("data-rs-state", "closed");
                if let Ok(Some(c)) = root_cb.query_selector("[data-rs-tooltip-content]") {
                    let _ = c.set_attribute("data-rs-state", "closed");
                }
            }));
            let _ = trigger.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // focus → open
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
                let _ = root_cb.set_attribute("data-rs-state", "open");
                if let Ok(Some(c)) = root_cb.query_selector("[data-rs-tooltip-content]") {
                    let _ = c.set_attribute("data-rs-state", "open");
                }
            }));
            let _ = trigger.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // blur → closed
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
                let _ = root_cb.set_attribute("data-rs-state", "closed");
                if let Ok(Some(c)) = root_cb.query_selector("[data-rs-tooltip-content]") {
                    let _ = c.set_attribute("data-rs-state", "closed");
                }
            }));
            let _ = trigger.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <span
            data-rs-tooltip=""
            data-rs-component="Tooltip"
            data-rs-state="closed"
            class=class
            node_ref=node_ref
        >
            <span
                data-rs-tooltip-trigger=""
                tabindex="0"
                aria-describedby="tooltip-content"
            >
                {label}
            </span>
            <span
                data-rs-tooltip-content=""
                data-rs-side=side
                data-rs-state="closed"
                role="tooltip"
                id="tooltip-content"
            >
                {content}
            </span>
        </span>
    }
}
