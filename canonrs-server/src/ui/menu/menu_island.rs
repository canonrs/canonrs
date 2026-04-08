//! Menu Island — Canon Rule #341
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn MenuIsland(
    items: Vec<MenuIslandItem>,
    #[prop(optional, into)] selected:   Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class:      Option<String>,
) -> impl IntoView {
    let class      = class.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let selected   = selected.unwrap_or_default();

    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::HtmlElement = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
            let state = item.get_attribute("data-rs-state").unwrap_or_default();
            if state.contains("disabled") { return; }

            // deselect all via sequential query_selector
            let mut idx = 0u32;
            loop {
                let sel = format!("[data-rs-menu-item]:nth-of-type({})", idx + 1);
                match root_cb.query_selector(&sel) {
                    Ok(Some(el)) => {
                        let _ = el.set_attribute("data-rs-state", "unselected");
                        let _ = el.set_attribute("aria-selected", "false");
                        idx += 1;
                    }
                    _ => break,
                }
            }
            let _ = item.set_attribute("data-rs-state", "selected");
            let _ = item.set_attribute("aria-selected", "true");
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let items_view = items.iter().map(|item| {
        let is_selected = item.value == selected;
        let state       = if is_selected { "selected" } else { "unselected" };
        let disabled    = item.disabled;
        let label       = item.label.clone();
        view! {
            <div
                data-rs-menu-item=""
                role="menuitem"
                data-rs-state=state
                aria-selected=is_selected.to_string()
                aria-disabled=disabled.to_string()
            >
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-menu=""
            data-rs-component="Menu"
            role="menu"
            aria-label=aria_label
            class=class
            node_ref=node_ref
        >
            {items_view}
        </div>
    }
}
