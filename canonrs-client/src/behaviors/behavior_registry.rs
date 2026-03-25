//! Behavior Registry - Auto-discovery e registro centralizado de behaviors

#[cfg(feature = "hydrate")]
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::window;
#[cfg(feature = "hydrate")]
use web_sys::{Document, HtmlElement, MutationObserver, MutationObserverInit, Node, NodeList, Element};
#[cfg(feature = "hydrate")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "hydrate")]
use canonrs_core::{BehaviorError, BehaviorResult};

#[cfg(feature = "hydrate")]
#[derive(Clone)]
pub struct ComponentState {
    pub open: RwSignal<bool>,
}

#[cfg(feature = "hydrate")]
pub type BehaviorFn = Box<dyn Fn(&web_sys::Element, &ComponentState) -> BehaviorResult<()>>;

#[cfg(feature = "hydrate")]
thread_local! {
    static BEHAVIOR_HANDLERS: std::sync::Mutex<Vec<(String, BehaviorFn)>> =
        std::sync::Mutex::new(Vec::new());
}

#[cfg(feature = "hydrate")]
pub fn register_behavior(data_attr: &str, handler: BehaviorFn) {
    BEHAVIOR_HANDLERS.with(|handlers| {
        handlers.lock().unwrap().push((data_attr.to_string(), handler));
    });
}

#[cfg(feature = "hydrate")]
pub fn init_behavior_registry() -> BehaviorResult<()> {
    let document: Document = window().document()
        .ok_or_else(|| BehaviorError::JsError { message: "Document not available".into() })?;

    let body: HtmlElement = document.body()
        .ok_or_else(|| BehaviorError::ElementNotFound { selector: "body".into() })?;

    scan_and_attach(&document)?;

    let callback = Closure::wrap(Box::new(move |_mutations: js_sys::Array, _observer: MutationObserver| {
        if let Some(doc) = window().document() {
            let _ = scan_and_attach(&doc);
        }
    }) as Box<dyn FnMut(_, _)>);

    let observer: MutationObserver = MutationObserver::new(callback.as_ref().unchecked_ref())
        .map_err(|_| BehaviorError::ObserverFailed { reason: "Failed to create observer".into() })?;

    let init = MutationObserverInit::new();
    init.set_child_list(true);
    init.set_subtree(true);

    observer.observe_with_options(&body, &init)
        .map_err(|_| BehaviorError::ObserverFailed { reason: "Failed to observe".into() })?;

    callback.forget();
    std::mem::forget(observer);

    Ok(())
}

#[cfg(feature = "hydrate")]
fn scan_and_attach(document: &Document) -> BehaviorResult<()> {
    BEHAVIOR_HANDLERS.with(|handlers| {
        let handlers = handlers.lock().unwrap();

        for (data_attr, handler) in handlers.iter() {
            let selector = format!("[{}]", data_attr);
            let elements: NodeList = document.query_selector_all(&selector)
                .map_err(|_| BehaviorError::ElementNotFound { selector: selector.clone() })?;

            for i in 0..elements.length() {
                let node: Node = match elements.item(i) {
                    Some(n) => n,
                    None => continue,
                };

                let element: Element = match node.dyn_into() {
                    Ok(el) => el,
                    Err(_) => continue,
                };

                // DOM como source of truth — sem REGISTRY, sem id
                if element.has_attribute("data-rs-initialized") {
                    continue;
                }

                element.set_attribute("data-rs-initialized", "1").ok();

                let state = ComponentState {
                    open: RwSignal::new(false),
                };

                if let Err(e) = handler(&element, &state) {
                    web_sys::console::error_1(
                        &format!("Behavior error on {:?}: {}", element.tag_name(), e).into()
                    );
                }
            }
        }
        Ok(())
    })
}
