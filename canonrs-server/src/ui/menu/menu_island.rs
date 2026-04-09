//! Menu Island — Canon Rule init (DOM-driven)
use leptos::prelude::*;

#[island]
pub fn MenuIsland(
    children: Children,
    #[prop(optional, into)] _selected:  Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class:      Option<String>,
) -> impl IntoView {
    let class      = class.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let node_ref   = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::HtmlElement = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let rc = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(item) = t.closest("[data-rs-menu-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
            let mut idx = 0u32;
            loop {
                match rc.query_selector(&format!("[data-rs-menu-item]:nth-of-type({})", idx+1)) {
                    Ok(Some(el)) => { let _ = el.set_attribute("data-rs-state", "unselected"); let _ = el.set_attribute("aria-selected", "false"); idx += 1; }
                    _ => break,
                }
            }
            let _ = item.set_attribute("data-rs-state", "selected");
            let _ = item.set_attribute("aria-selected", "true");
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    view! {
        <div
            data-rs-menu=""
            data-rs-component="Menu"
            role="menu"
            aria-label=aria_label
            class=class
            node_ref=node_ref
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuItemIsland(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = if selected { "selected" } else { "unselected" };
    view! {
        <div
            data-rs-menu-item=""
            role="menuitem"
            data-rs-state=state
            data-rs-value=value
            aria-selected=selected.to_string()
            aria-disabled=disabled.to_string()
        >
            {children()}
        </div>
    }
}
