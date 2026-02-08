use leptos::prelude::*;
use leptos::web_sys::{EventTarget, HtmlElement, Element, Window, CssStyleDeclaration};
use wasm_bindgen::{prelude::*, JsCast};

/// ListItem behavior - single-select list functionality
pub struct ListItemBehavior {
    item_ids: Vec<String>,
}

impl ListItemBehavior {
    pub fn new(item_ids: Vec<&str>) -> Self {
        Self {
            item_ids: item_ids.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn attach(&self) {
        let item_ids = self.item_ids.clone();

        Effect::new(move |_| {
            let win: Window = leptos::leptos_dom::helpers::window();
            let document = win.document().expect("document not available");

            for item_id in &item_ids {
                if let Some(item) = document.get_element_by_id(item_id) {
                    let item_elem: Element = item;
                    let doc = document.clone();
                    let all_ids = item_ids.clone();
                    let current_id = item_id.clone();

                    let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                        for other_id in &all_ids {
                            if let Some(other) = doc.get_element_by_id(other_id) {
                                let other_elem: Element = other;
                                other_elem.set_attribute("data-selected", "false").ok();
                                let other_html: &HtmlElement = match other_elem.dyn_ref::<HtmlElement>() {
                                    Some(el) => el,
                                    None => continue,
                                };
                                let other_style: CssStyleDeclaration = other_html.style();
                                other_style.set_property("background", "transparent").ok();
                            }
                        }

                        if let Some(current) = doc.get_element_by_id(&current_id) {
                            let current_elem: Element = current;
                            current_elem.set_attribute("data-selected", "true").ok();
                            let current_html: &HtmlElement = match current_elem.dyn_ref::<HtmlElement>() {
                                Some(el) => el,
                                None => return,
                            };
                            let current_style: CssStyleDeclaration = current_html.style();
                            current_style.set_property("background", "var(--semantic-action-primary-bg-subtle)").ok();
                        }
                    }) as Box<dyn FnMut(_)>);

                    let target: &EventTarget = item_elem.as_ref();
                    target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
                    closure.forget();
                }
            }
        });
    }
}
