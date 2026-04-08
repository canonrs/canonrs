//! VirtualList Interaction Engine — scroll-driven virtualization

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    let items_count = root.get_attribute("data-items-count")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let item_height = root.get_attribute("data-item-height")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(40.0);

    let total_height = items_count as f64 * item_height;

    // create viewport + content structure
    let script = format!(
        r#"
        var root = arguments[0];
        var vh = root.clientHeight || 400;
        root.style.height = vh + 'px';
        root.style.overflowY = 'auto';
        root.style.position = 'relative';

        var content = document.createElement('div');
        content.setAttribute('data-rs-virtual-list-content', '');
        content.style.height = '{total}px';
        content.style.position = 'relative';
        root.appendChild(content);

        function render() {{
            var st = root.scrollTop;
            var start = Math.max(0, Math.floor(st / {ih}) - 2);
            var end = Math.min({ic}, Math.ceil((st + vh) / {ih}) + 2);
            content.innerHTML = '';
            for (var i = start; i < end; i++) {{
                var el = document.createElement('div');
                el.setAttribute('data-rs-virtual-list-item', '');
                el.style.position = 'absolute';
                el.style.top = (i * {ih}) + 'px';
                el.style.height = '{ih}px';
                el.style.width = '100%';
                el.style.display = 'flex';
                el.style.alignItems = 'center';
                el.textContent = 'Item ' + (i + 1);
                content.appendChild(el);
            }}
        }}

        render();
        root.addEventListener('scroll', render, {{ passive: true }});
        "#,
        total = total_height,
        ih = item_height,
        ic = items_count,
    );

    let f = js_sys::Function::new_with_args("arguments", &script);
    f.call1(&JsValue::NULL, &root).ok();
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-virtual-list]") { Ok(n) => n, Err(_) => return };
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
    opts.set_child_list(true);
    opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
}
