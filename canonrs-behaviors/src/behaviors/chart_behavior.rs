use leptos::prelude::*;
use leptos::leptos_dom::helpers::window;
use web_sys::{Document, EventTarget, HtmlInputElement, HtmlElement, Element, Node, NodeList, CssStyleDeclaration};
use wasm_bindgen::{prelude::*, JsCast};

/// Chart behavior - interactive chart with series toggle and hover values
pub struct ChartBehavior {
    series: Vec<(String, String)>,
}

impl ChartBehavior {
    pub fn new(series: Vec<(&str, &str)>) -> Self {
        Self {
            series: series.iter().map(|(id, class)| (id.to_string(), class.to_string())).collect(),
        }
    }

    pub fn attach(&self) {
        let series = self.series.clone();

        Effect::new(move |_| {
            let document: Document = window().document().expect("document");

            // Initialize checkboxes
            for (toggle_id, _) in &series {
                let element: Element = match document.get_element_by_id(toggle_id) {
                    Some(el) => el,
                    None => continue,
                };

                let input: &HtmlInputElement = match element.dyn_ref::<HtmlInputElement>() {
                    Some(inp) => inp,
                    None => continue,
                };

                input.set_checked(true);
            }

            // Toggle series visibility
            for (toggle_id, class_name) in &series {
                let checkbox_element: Element = match document.get_element_by_id(toggle_id) {
                    Some(el) => el,
                    None => continue,
                };

                let doc: Document = document.clone();
                let class: String = class_name.clone();

                let closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    let target_et: EventTarget = match e.target() {
                        Some(t) => t,
                        None => return,
                    };

                    let input: HtmlInputElement = match target_et.dyn_into() {
                        Ok(inp) => inp,
                        Err(_) => return,
                    };

                    let checked: bool = input.checked();
                    let display: &str = if checked { "block" } else { "none" };

                    let bars: NodeList = match doc.query_selector_all(&format!(".chart-bar.{}", class)) {
                        Ok(b) => b,
                        Err(_) => return,
                    };

                    let length: u32 = bars.length();
                    for i in 0..length {
                        let node: Node = match bars.item(i) {
                            Some(n) => n,
                            None => continue,
                        };

                        let bar: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                            Some(b) => b,
                            None => continue,
                        };

                        let style: CssStyleDeclaration = bar.style();
                        let _ = style.set_property("display", display);
                    }
                }) as Box<dyn FnMut(_)>);

                let target: &EventTarget = checkbox_element.as_ref();
                let _ = target.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget();
            }

            // Hover to show values
            let bars: NodeList = match document.query_selector_all(".chart-bar") {
                Ok(b) => b,
                Err(_) => return,
            };

            let length: u32 = bars.length();
            for i in 0..length {
                let node: Node = match bars.item(i) {
                    Some(n) => n,
                    None => continue,
                };

                let show_closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    let target_et: EventTarget = match e.current_target() {
                        Some(t) => t,
                        None => return,
                    };

                    let bar_elem: Element = match target_et.dyn_into() {
                        Ok(el) => el,
                        Err(_) => return,
                    };

                    let label: Element = match bar_elem.query_selector(".bar-label") {
                        Ok(Some(l)) => l,
                        _ => return,
                    };

                    let label_html: &HtmlElement = match label.dyn_ref::<HtmlElement>() {
                        Some(lh) => lh,
                        None => return,
                    };

                    let style: CssStyleDeclaration = label_html.style();
                    let _ = style.set_property("display", "block");
                }) as Box<dyn FnMut(_)>);

                let target: &EventTarget = node.as_ref();
                let _ = target.add_event_listener_with_callback("mouseenter", show_closure.as_ref().unchecked_ref());
                show_closure.forget();

                let node_clone: Node = node.clone();
                let hide_closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    let target_et: EventTarget = match e.current_target() {
                        Some(t) => t,
                        None => return,
                    };

                    let bar_elem: Element = match target_et.dyn_into() {
                        Ok(el) => el,
                        Err(_) => return,
                    };

                    let label: Element = match bar_elem.query_selector(".bar-label") {
                        Ok(Some(l)) => l,
                        _ => return,
                    };

                    let label_html: &HtmlElement = match label.dyn_ref::<HtmlElement>() {
                        Some(lh) => lh,
                        None => return,
                    };

                    let style: CssStyleDeclaration = label_html.style();
                    let _ = style.set_property("display", "none");
                }) as Box<dyn FnMut(_)>);

                let target2: &EventTarget = node_clone.as_ref();
                let _ = target2.add_event_listener_with_callback("mouseleave", hide_closure.as_ref().unchecked_ref());
                hide_closure.forget();
            }
        });
    }
}
