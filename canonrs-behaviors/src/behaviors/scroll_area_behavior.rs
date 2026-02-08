use leptos::prelude::*;
use leptos::leptos_dom::helpers::window;
use leptos::web_sys::{Document, Element, EventTarget, HtmlElement};
use wasm_bindgen::{prelude::*, JsCast};

/// ScrollArea behavior - tracks scroll percentage
pub struct ScrollAreaBehavior {
    area_id: String,
    indicator_id: String,
}

impl ScrollAreaBehavior {
    pub fn new(area_id: &str, indicator_id: &str) -> Self {
        Self {
            area_id: area_id.to_string(),
            indicator_id: indicator_id.to_string(),
        }
    }

    pub fn attach(&self) {
        let area_id = self.area_id.clone();
        let indicator_id = self.indicator_id.clone();

        Effect::new(move |_| {
            let document: Document = match window().document() {
                Some(d) => d,
                None => return,
            };

            let element: Element = match document.get_element_by_id(&area_id) {
                Some(el) => el,
                None => return,
            };

            let scroll_area: HtmlElement = match element.dyn_into() {
                Ok(html_el) => html_el,
                Err(_) => return,
            };

            let doc: Document = document.clone();
            let indicator: String = indicator_id.clone();

            let closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                let target_et: EventTarget = match e.target() {
                    Some(t) => t,
                    None => return,
                };

                let elem: HtmlElement = match target_et.dyn_into() {
                    Ok(el) => el,
                    Err(_) => return,
                };

                let scroll_top: f64 = elem.scroll_top() as f64;
                let scroll_height: f64 = elem.scroll_height() as f64;
                let client_height: f64 = elem.client_height() as f64;

                let percentage: f64 = if scroll_height > client_height {
                    ((scroll_top / (scroll_height - client_height)) * 100.0).round()
                } else {
                    0.0
                };

                let ind_element: Element = match doc.get_element_by_id(&indicator) {
                    Some(el) => el,
                    None => return,
                };

                ind_element.set_inner_html(&format!("Scroll: {}%", percentage));
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = scroll_area.as_ref();
            let _ = target.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }
}
