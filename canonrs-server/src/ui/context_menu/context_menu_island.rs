//! ContextMenu Island — Canon Rule init (DOM-driven)
use leptos::prelude::*;

#[island]
pub fn ContextMenuIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class         = class.unwrap_or_default();
    let node_ref      = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let content_sel = "[data-rs-context-menu-content]";

        // contextmenu → show
        { let rc = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.prevent_default(); e.stop_propagation();
            if let Ok(Some(content)) = rc.query_selector(content_sel) {
                let _ = content.set_attribute("data-rs-state", "open");
                let _ = content.set_attribute("style", &format!("left:{}px;top:{}px;", e.client_x(), e.client_y()));
                if let Ok(el) = content.dyn_into::<web_sys::HtmlElement>() { el.set_hidden(false); }
            }
        }));
        let _ = root.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref());
        cb.forget(); }

        // click outside → close
        { let rc = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if !rc.contains(Some(&t)) {
                if let Ok(Some(content)) = rc.query_selector(content_sel) {
                    let _ = content.set_attribute("data-rs-state", "closed");
                    if let Ok(el) = content.dyn_into::<web_sys::HtmlElement>() { el.set_hidden(true); }
                }
            }
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget(); }
    });

    view! {
        <div
            data-rs-context-menu=""
            data-rs-component="ContextMenu"
            data-rs-state="closed"
            class=class
            node_ref=node_ref
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuTriggerIsland(children: Children) -> impl IntoView {
    view! { <div data-rs-context-menu-trigger="">{children()}</div> }
}

#[component]
pub fn ContextMenuContentIsland(children: Children) -> impl IntoView {
    view! {
        <div data-rs-context-menu-content="" data-rs-state="closed" role="menu" hidden=true>
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuItemIsland(
    children: Children,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <div data-rs-context-menu-item="" role="menuitem" aria-disabled=disabled.to_string()>
            {children()}
        </div>
    }
}
