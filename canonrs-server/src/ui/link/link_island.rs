//! @canon-level: strict
//! Link Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;
use super::link_ui::Link;
use canonrs_core::primitives::LinkVariant;

#[island]
pub fn LinkIsland(
    children: Children,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] variant: Option<LinkVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] external: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let href     = href.unwrap_or_default();
    let variant  = variant.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let external = external.unwrap_or(false);
    let class    = class.unwrap_or_default();

    let node_ref = NodeRef::<leptos::html::A>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(el_html) = node_ref.get() else { return };
        let el: web_sys::Element = (*el_html).clone().unchecked_into();

        {
            let el_cb = el.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                if !disabled {
                    let state = el_cb.get_attribute("data-rs-state").unwrap_or_default();
                    let next = format!("{} hover", state).trim().to_string();
                    let _ = el_cb.set_attribute("data-rs-state", &next);
                }
            }));
            let _ = el.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let el_cb = el.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                let state = el_cb.get_attribute("data-rs-state").unwrap_or_default();
                let next = state.split_whitespace()
                    .filter(|t| *t != "hover" && *t != "active")
                    .collect::<Vec<_>>().join(" ");
                let _ = el_cb.set_attribute("data-rs-state", &next);
            }));
            let _ = el.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let el_cb = el.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                if !disabled {
                    let state = el_cb.get_attribute("data-rs-state").unwrap_or_default();
                    let next = format!("{} active", state).trim().to_string();
                    let _ = el_cb.set_attribute("data-rs-state", &next);
                }
            }));
            let _ = el.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let el_cb = el.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                let state = el_cb.get_attribute("data-rs-state").unwrap_or_default();
                let next = state.split_whitespace()
                    .filter(|t| *t != "active")
                    .collect::<Vec<_>>().join(" ");
                let _ = el_cb.set_attribute("data-rs-state", &next);
            }));
            let _ = el.add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <Link
            href=href
            variant=variant
            disabled=disabled
            external=external
            class=class
            node_ref=node_ref
        >
            {children()}
        </Link>
    }
}
