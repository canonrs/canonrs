#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
pub fn init_tabs() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::window;
    use web_sys::{Document, HtmlElement, Node, NodeList, Element, EventTarget};
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let document: Document = window().document().expect("document");

        let tabs_list: NodeList = match document.query_selector_all("[role='tablist'] [role='tab']") {
            Ok(list) => list,
            Err(_) => return,
        };
        let length: u32 = tabs_list.length();

        for i in 0..length {
            let node: Node = match tabs_list.item(i) {
                Some(n) => n,
                None => continue,
            };

            let tab_el: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                Some(el) => el,
                None => continue,
            };

            let tab_id: String = tab_el.get_attribute("data-tab").unwrap_or_default();

            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let doc: Document = window().document().expect("document");

                let all_tabs: NodeList = match doc.query_selector_all("[role='tab']") {
                    Ok(list) => list,
                    Err(_) => return,
                };
                let tabs_length: u32 = all_tabs.length();
                for j in 0..tabs_length {
                    let t_node: Node = match all_tabs.item(j) {
                        Some(n) => n,
                        None => continue,
                    };
                    let t_el: &HtmlElement = match t_node.dyn_ref::<HtmlElement>() {
                        Some(el) => el,
                        None => continue,
                    };
                    let _ = t_el.set_attribute("aria-selected", "false");
                }

                let all_panels: NodeList = match doc.query_selector_all("[role='tabpanel']") {
                    Ok(list) => list,
                    Err(_) => return,
                };
                let panels_length: u32 = all_panels.length();
                for j in 0..panels_length {
                    let p_node: Node = match all_panels.item(j) {
                        Some(n) => n,
                        None => continue,
                    };
                    let p_el: &HtmlElement = match p_node.dyn_ref::<HtmlElement>() {
                        Some(el) => el,
                        None => continue,
                    };
                    let _ = p_el.set_attribute("hidden", "");
                }

                let selected_tab: Element = match doc.query_selector(&format!("[data-tab='{}']", tab_id)) {
                    Ok(Some(t)) => t,
                    _ => return,
                };
                let _ = selected_tab.set_attribute("aria-selected", "true");

                let selected_panel: Element = match doc.query_selector(&format!("[data-panel='{}']", tab_id)) {
                    Ok(Some(p)) => p,
                    _ => return,
                };
                let _ = selected_panel.remove_attribute("hidden");
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = tab_el.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn init_tabs() {}
