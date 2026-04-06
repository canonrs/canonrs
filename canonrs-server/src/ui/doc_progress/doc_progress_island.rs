use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use super::doc_progress_ui::{DocProgress, DocProgressSlot, DocProgressSlotPosition};

fn setup_tracking(progress_selector: &str, target_id: String) {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    let progress_el: web_sys::Element = match doc.query_selector(progress_selector) {
        Ok(Some(e)) => e,
        _ => return,
    };

    if progress_el.get_attribute("data-rs-attached").as_deref() == Some("1") { return; }
    progress_el.set_attribute("data-rs-attached", "1").ok();

    let container: Option<web_sys::HtmlElement> = if !target_id.is_empty() {
        doc.get_element_by_id(&target_id)
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    } else {
        None
    };

    let update = {
        let progress_el = progress_el.clone();
        let container = container.clone();
        move || {
            let win = match web_sys::window() { Some(w) => w, None => return };
            let doc = match win.document() { Some(d) => d, None => return };
            let (scroll_top, scroll_height, client_height) = if let Some(ref c) = container {
                (c.scroll_top() as f64, c.scroll_height() as f64, c.client_height() as f64)
            } else {
                let doc_el = match doc.document_element() { Some(d) => d, None => return };
                (
                    win.scroll_y().unwrap_or(0.0),
                    doc_el.scroll_height() as f64,
                    win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0),
                )
            };
            let max_scroll = scroll_height - client_height;
            if max_scroll <= 0.0 { return; }
            let pct = ((scroll_top / max_scroll) * 100.0).clamp(0.0, 100.0);
            progress_el.set_attribute("data-progress", &format!("{:.0}", pct)).ok();
            if let Ok(Some(bar)) = progress_el.query_selector("[data-rs-doc-progress-bar]") {
                if let Ok(bar_el) = bar.dyn_into::<web_sys::HtmlElement>() {
                    bar_el.style().set_property("width", &format!("{}%", pct)).ok();
                }
            }
        }
    };

    let update_clone = update.clone();
    let handler = Closure::wrap(Box::new(move || { update_clone(); }) as Box<dyn FnMut()>);
    if let Some(ref c) = container {
        c.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref()).ok();
    } else {
        win.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref()).ok();
    }
    handler.forget();

    let init = Closure::once_into_js(move || { update(); });
    win.set_timeout_with_callback_and_timeout_and_arguments_0(init.unchecked_ref(), 100).ok();
}

#[island]
pub fn DocProgressIsland(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] scroll_target: Option<String>,
) -> impl IntoView {
    let root = NodeRef::<leptos::html::Div>::new();
    let cls = class.unwrap_or_default();
    let target_id = scroll_target.unwrap_or_default();

    Effect::new(move |_| {
        let el = match root.get() { Some(e) => e, None => return };
        if el.has_attribute("data-rs-attached") { return; }
        el.set_attribute("data-rs-attached", "1").ok();
        setup_tracking("[data-rs-doc-progress]", target_id.clone());
    });

    view! {
        <div node_ref=root>
            <DocProgress class=cls />
        </div>
    }
}

#[island]
pub fn DocProgressSlotIsland(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] scroll_target: Option<String>,
    #[prop(optional, into)] position: Option<String>,
) -> impl IntoView {
    let root = NodeRef::<leptos::html::Div>::new();
    let cls = class.unwrap_or_default();
    let target_id = scroll_target.clone().unwrap_or_default();
    let scroll_target_prop = scroll_target.unwrap_or_default();
    let position = match position.as_deref() {
        Some("bottom") => DocProgressSlotPosition::Bottom,
        _ => DocProgressSlotPosition::Top,
    };

    Effect::new(move |_| {
        let el = match root.get() { Some(e) => e, None => return };
        if el.has_attribute("data-rs-attached") { return; }
        el.set_attribute("data-rs-attached", "1").ok();
        setup_tracking("[data-rs-doc-progress-portal]", target_id.clone());
    });

    view! {
        <div node_ref=root>
            <DocProgressSlot class=cls scroll_target=scroll_target_prop position=position />
        </div>
    }
}
