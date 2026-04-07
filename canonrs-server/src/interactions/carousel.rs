//! Carousel Interaction Engine
//! Slide navigation, keyboard, autoplay, indicators — CR-338 / CR-339

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn get_current_index(carousel: &Element) -> usize {
    carousel.get_attribute("data-rs-current-index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)
}

fn go_to_slide(carousel: &Element, index: usize) {
    carousel.set_attribute("data-rs-current-index", &index.to_string()).ok();

    if let Ok(items) = carousel.query_selector_all("[data-rs-carousel-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    if i == index as u32 {
                        item.set_attribute("data-rs-state", "active").ok();
                        item.remove_attribute("hidden").ok();
                        item.set_attribute("aria-hidden", "false").ok();
                    } else {
                        item.set_attribute("data-rs-state", "inactive").ok();
                        item.set_attribute("hidden", "").ok();
                        item.set_attribute("aria-hidden", "true").ok();
                    }
                }
            }
        }
    }

    if let Ok(dots) = carousel.query_selector_all("[data-rs-carousel-dot]") {
        for i in 0..dots.length() {
            if let Some(node) = dots.item(i) {
                if let Ok(dot) = node.dyn_into::<Element>() {
                    if i == index as u32 {
                        dot.set_attribute("data-rs-state", "active").ok();
                        dot.set_attribute("aria-current", "true").ok();
                    } else {
                        dot.set_attribute("data-rs-state", "inactive").ok();
                        dot.remove_attribute("aria-current").ok();
                    }
                }
            }
        }
    }

    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(index as f64)).ok();
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&JsValue::from(&detail));
    if let Ok(ev) = web_sys::CustomEvent::new_with_event_init_dict("canon:carousel-change", &init) {
        carousel.dispatch_event(&ev).ok();
    }
}

fn prev_slide(carousel: &Element, total: usize, loop_mode: bool) {
    let current = get_current_index(carousel);
    let new_index = if current == 0 {
        if loop_mode { total - 1 } else { 0 }
    } else { current - 1 };
    go_to_slide(carousel, new_index);
}

fn next_slide(carousel: &Element, total: usize, loop_mode: bool) {
    let current = get_current_index(carousel);
    let new_index = if current >= total - 1 {
        if loop_mode { 0 } else { total - 1 }
    } else { current + 1 };
    go_to_slide(carousel, new_index);
}

pub fn init(root: Element) {
    let key = JsValue::from_str("__rs_carousel_bound");
    let root_val = JsCast::unchecked_ref::<JsValue>(&root);
    if js_sys::Reflect::get(root_val, &key).ok().filter(|v| v.is_truthy()).is_some() { return; }
    js_sys::Reflect::set(root_val, &key, &JsValue::TRUE).ok();

    let Ok(Some(wrapper_node)) = root.query_selector("[data-rs-carousel-wrapper]") else { return };
    let Ok(wrapper) = wrapper_node.dyn_into::<web_sys::HtmlElement>() else { return };

    let initial_index = wrapper.get_attribute("data-rs-initial-index")
        .and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let autoplay = wrapper.has_attribute("data-rs-autoplay");
    let loop_mode = wrapper.has_attribute("data-rs-loop");
    let interval = wrapper.get_attribute("data-rs-interval")
        .and_then(|s| s.parse::<i32>().ok()).unwrap_or(5000);

    let Ok(items) = root.query_selector_all("[data-rs-carousel-item]") else { return };
    let total = items.length() as usize;
    if total == 0 { return; }

    // indicators
    if let Ok(Some(indicators)) = root.query_selector("[data-rs-carousel-indicators]") {
        let doc = web_sys::window().unwrap().document().unwrap();
        for i in 0..total {
            let Ok(dot) = doc.create_element("button") else { continue };
            dot.set_attribute("data-rs-carousel-dot", "").ok();
            dot.set_attribute("data-rs-index", &i.to_string()).ok();
            dot.set_attribute("data-rs-state", "inactive").ok();
            dot.set_attribute("aria-label", &format!("Go to slide {}", i + 1)).ok();
            let root_c = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                go_to_slide(&root_c, i);
            }) as Box<dyn FnMut(_)>);
            dot.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
            indicators.append_child(&dot).ok();
        }
    }

    // marcar como inicializado — CSS pode agora confiar nos states
    root.set_attribute("data-rs-carousel-ready", "").ok();
    go_to_slide(&root, initial_index);

    // prev
    if let Ok(Some(prev)) = root.query_selector("[data-rs-carousel-prev]") {
        let root_c = root.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            prev_slide(&root_c, total, loop_mode);
        }) as Box<dyn FnMut(_)>);
        prev.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // next
    if let Ok(Some(next)) = root.query_selector("[data-rs-carousel-next]") {
        let root_c = root.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            next_slide(&root_c, total, loop_mode);
        }) as Box<dyn FnMut(_)>);
        next.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // keyboard
    {
        let root_c = root.clone();
        let kb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            match e.key().as_str() {
                "ArrowLeft"  => { e.prevent_default(); prev_slide(&root_c, total, loop_mode); }
                "ArrowRight" => { e.prevent_default(); next_slide(&root_c, total, loop_mode); }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("keydown", kb.as_ref().unchecked_ref()).ok();
        kb.forget();
    }

    // autoplay
    if autoplay {
        let root_c = root.clone();
        let cb = Closure::wrap(Box::new(move || {
            next_slide(&root_c, total, true);
        }) as Box<dyn FnMut()>);
        web_sys::window().unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), interval).ok();
        cb.forget();
    }
}

fn try_init_all(doc: &web_sys::Document) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-carousel]") else { return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    try_init_all(&doc);
    let doc_obs = doc.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        try_init_all(&doc_obs);
    }) as Box<dyn FnMut(_, _)>);
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o, Err(_) => { cb.forget(); return }
    };
    let opts = web_sys::MutationObserverInit::new();
    opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
    let obs_clone = observer.clone();
    let disconnect = Closure::wrap(Box::new(move || { obs_clone.disconnect(); }) as Box<dyn Fn()>);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
