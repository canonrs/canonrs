#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement, Element, Event};

#[cfg(feature = "hydrate")]
fn close_all(root: &web_sys::Element) {
    if let Ok(menus) = root.query_selector_all("[data-rs-menubar-menu]") {
        for i in 0..menus.length() {
            if let Some(node) = menus.item(i) {
                if let Ok(menu) = node.dyn_into::<Element>() {
                    menu.set_attribute("data-rs-state", "closed").ok();
                    if let Ok(Some(trigger)) = menu.query_selector("[data-rs-menubar-trigger]") {
                        trigger.set_attribute("aria-expanded", "false").ok();
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-menubar", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-menubar-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-menubar-attached", "1").ok();

        let triggers = root.query_selector_all("[data-rs-menubar-trigger]")
            .map_err(|_| canonrs_core::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let Some(node) = triggers.item(i) else { continue };
            let Ok(trigger) = node.dyn_into::<HtmlElement>() else { continue };

            if trigger.get_attribute("data-rs-menubar-trigger-initialized").is_some() { continue; }
            trigger.set_attribute("data-rs-menubar-trigger-initialized", "1").ok();

            let root_click = root.clone();
            let trigger_click = trigger.clone();

            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let menu = match trigger_click.closest("[data-rs-menubar-menu]").ok().flatten() {
                    Some(el) => el,
                    None => return,
                };
                let is_open = menu.get_attribute("data-rs-state").as_deref() == Some("open");

                // fechar todos
                close_all(&root_click);

                // se estava fechado, abrir + posicionar
                if !is_open {
                    menu.set_attribute("data-rs-state", "open").ok();
                    trigger_click.set_attribute("aria-expanded", "true").ok();

                    if let Ok(Some(content)) = menu.query_selector("[data-rs-menubar-content]") {
                        if let Ok(content_el) = content.dyn_into::<HtmlElement>() {
                            let rect = trigger_click.get_bounding_client_rect();
                            content_el.style().set_property("left", &format!("{}px", rect.left())).ok();
                            content_el.style().set_property("top", &format!("{}px", rect.bottom() + 4.0)).ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // fechar ao clicar fora
        let root_outside = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                if target.closest("[data-rs-menubar]").ok().flatten().is_none() {
                    close_all(&root_outside);
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        // fechar ao rolar
        let root_scroll = root.clone();
        let cb_scroll = Closure::wrap(Box::new(move |_: Event| {
            close_all(&root_scroll);
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("scroll", cb_scroll.as_ref().unchecked_ref()).ok();
        cb_scroll.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
