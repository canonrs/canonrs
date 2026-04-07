//! @canon-level: strict
//! Toolbar Island — WAI-ARIA toolbar pattern, roving tabindex only

use leptos::prelude::*;
use super::toolbar_ui::{Toolbar, ToolbarSeparator};
use canonrs_core::primitives::ToolbarOrientation;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolbarItem {
    pub label:           String,
    pub value:           String,
    pub disabled:        bool,
    pub separator_after: bool,
}

static TOOLBAR_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn ToolbarIsland(
    items: Vec<ToolbarItem>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] orientation: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let aria_label  = aria_label.unwrap_or_else(|| "Toolbar".to_string());
    let is_vertical = orientation.as_deref() == Some("vertical");
    let orient      = if is_vertical { ToolbarOrientation::Vertical } else { ToolbarOrientation::Horizontal };
    let class       = class.unwrap_or_default();
    let node_ref    = NodeRef::<leptos::html::Div>::new();
    let _id         = TOOLBAR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        if root.has_attribute("data-rs-toolbar-js") { return; }
        root.set_attribute("data-rs-toolbar-js", "1").ok();

        fn get_items(root: &web_sys::Element) -> Vec<web_sys::Element> {
            let mut result = Vec::new();
            let mut i = 0u32;
            loop {
                let selector = format!("[data-rs-toolbar-item]:nth-of-type({})", i + 1);
                match root.query_selector(&selector) {
                    Ok(Some(el)) => { result.push(el); i += 1; }
                    _ => break,
                }
            }
            result
        }

        // roving tabindex init
        let items_vec = get_items(&root);
        for (i, item) in items_vec.iter().enumerate() {
            item.set_attribute("tabindex", if i == 0 { "0" } else { "-1" }).ok();
        }

        // keydown — roving tabindex only, NO state changes
        {
            let root_c = root.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-toolbar-item]").ok().flatten().is_none() { return; }
                let items = get_items(&root_c);
                let len = items.len();
                if len == 0 { return; }
                let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) else { return };
                let next = match e.key().as_str() {
                    "ArrowRight" if !is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                    "ArrowLeft"  if !is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                    "ArrowDown"  if  is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                    "ArrowUp"    if  is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                    "Home" => { e.prevent_default(); Some(0) }
                    "End"  => { e.prevent_default(); Some(len - 1) }
                    _ => None,
                };
                if let Some(next_idx) = next {
                    for (i, item) in items.iter().enumerate() {
                        item.set_attribute("tabindex", if i == next_idx { "0" } else { "-1" }).ok();
                    }
                    if let Ok(el) = items[next_idx].clone().dyn_into::<web_sys::HtmlElement>() {
                        el.focus().ok();
                    }
                }
            }));
            root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // click — toggle aria-pressed + dispatch event
        {
            use leptos::wasm_bindgen::JsValue;
            let root_c = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                let Some(item) = target.closest("[data-rs-toolbar-item]").ok().flatten() else { return };
                let pressed = item.get_attribute("aria-pressed").as_deref() == Some("true");
                let next = if pressed { "false" } else { "true" };
                item.set_attribute("aria-pressed", next).ok();
                // dispatch via JS eval — evita dependencias de js_sys
                let value = item.get_attribute("data-rs-value").unwrap_or_default();
                let script = format!(
                    "arguments[0].dispatchEvent(new CustomEvent('canon:toolbar:action',{{bubbles:true,detail:{{value:'{}',pressed:{}}}}}));",
                    value, next == "true"
                );
                let f = leptos::web_sys::js_sys::Function::new_with_args("arguments", &script);
                f.call1(&JsValue::NULL, &root_c).ok();
            }));
            root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }
    });

    let items_view = items.into_iter().map(|item| {
        let sep = item.separator_after;
        let disabled = item.disabled;
        view! {
            <>
                <button
                    type="button"
                    data-rs-toolbar-item=""
                    data-rs-value=item.value
                    disabled=disabled
                    tabindex="-1"
                >
                    {item.label}
                </button>
                {if sep { Some(view! { <ToolbarSeparator /> }) } else { None }}
            </>
        }
    }).collect::<Vec<_>>();

    view! {
        <div node_ref=node_ref style="display:contents;">
            <Toolbar aria_label=aria_label orientation=orient class=class>
                {items_view}
            </Toolbar>
        </div>
    }
}
