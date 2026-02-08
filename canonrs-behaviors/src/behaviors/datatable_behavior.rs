use leptos::prelude::*;
use leptos::leptos_dom::helpers::window;
use web_sys::{Document, EventTarget, HtmlInputElement, Element, HtmlElement, Node, NodeList, CssStyleDeclaration};
use wasm_bindgen::{prelude::*, JsCast};

/// DataTable behavior - filter, sort, column toggle, and row actions
pub struct DataTableBehavior {
    filter_id: String,
    sort_id: String,
    tbody_id: String,
    col_ids: Vec<String>,
}

impl DataTableBehavior {
    pub fn new(filter: &str, sort: &str, tbody: &str, columns: Vec<&str>) -> Self {
        Self {
            filter_id: filter.to_string(),
            sort_id: sort.to_string(),
            tbody_id: tbody.to_string(),
            col_ids: columns.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn attach(&self) {
        let filter_id = self.filter_id.clone();
        let sort_id = self.sort_id.clone();
        let tbody_id = self.tbody_id.clone();
        let col_ids = self.col_ids.clone();

        Effect::new(move |_| {
            let document: Document = window().document().expect("document");

            // Initialize column checkboxes
            for col_id in &col_ids {
                let checkbox: Element = match document.get_element_by_id(col_id) {
                    Some(el) => el,
                    None => continue,
                };

                let input: &HtmlInputElement = match checkbox.dyn_ref::<HtmlInputElement>() {
                    Some(inp) => inp,
                    None => continue,
                };

                input.set_checked(true);
            }

            // Column visibility toggles
            for col_id in &col_ids {
                let checkbox: Element = match document.get_element_by_id(col_id) {
                    Some(el) => el,
                    None => continue,
                };

                let doc: Document = document.clone();
                let col_class: String = format!(".{}", col_id);

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
                    let display: &str = if checked { "table-cell" } else { "none" };

                    let cells: NodeList = match doc.query_selector_all(&col_class) {
                        Ok(c) => c,
                        Err(_) => return,
                    };

                    let length: u32 = cells.length();
                    for i in 0..length {
                        let node: Node = match cells.item(i) {
                            Some(n) => n,
                            None => continue,
                        };

                        let cell: &HtmlElement = match node.dyn_ref::<HtmlElement>() {
                            Some(c) => c,
                            None => continue,
                        };

                        let style: CssStyleDeclaration = cell.style();
                        let _ = style.set_property("display", display);
                    }
                }) as Box<dyn FnMut(_)>);

                let target: &EventTarget = checkbox.as_ref();
                let _ = target.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget();
            }

            // Filter
            let filter_input: Element = match document.get_element_by_id(&filter_id) {
                Some(el) => el,
                None => return,
            };

            let doc: Document = document.clone();

            let filter_closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                let target_et: EventTarget = match e.target() {
                    Some(t) => t,
                    None => return,
                };

                let input: HtmlInputElement = match target_et.dyn_into() {
                    Ok(inp) => inp,
                    Err(_) => return,
                };

                let filter_value: String = input.value().to_lowercase();

                let rows: NodeList = match doc.query_selector_all("[data-data-table-row]") {
                    Ok(r) => r,
                    Err(_) => return,
                };

                let length: u32 = rows.length();
                for i in 0..length {
                    let node: Node = match rows.item(i) {
                        Some(n) => n,
                        None => continue,
                    };

                    let row: &Element = match node.dyn_ref::<Element>() {
                        Some(r) => r,
                        None => continue,
                    };

                    let product: String = row.get_attribute("data-product").unwrap_or_default();
                    let matches: bool = product.to_lowercase().contains(&filter_value);

                    let html_row: &HtmlElement = match row.dyn_ref::<HtmlElement>() {
                        Some(hr) => hr,
                        None => continue,
                    };

                    let style: CssStyleDeclaration = html_row.style();
                    let display: &str = if matches { "table-row" } else { "none" };
                    let _ = style.set_property("display", display);
                }
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = filter_input.as_ref();
            let _ = target.add_event_listener_with_callback("input", filter_closure.as_ref().unchecked_ref());
            filter_closure.forget();

            // Sort
            let sort_btn: Element = match document.get_element_by_id(&sort_id) {
                Some(el) => el,
                None => return,
            };

            let doc2: Document = document.clone();
            let tbody: String = tbody_id.clone();
            let sort_asc = std::rc::Rc::new(std::cell::Cell::new(true));

            let sort_closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let asc: bool = sort_asc.get();
                sort_asc.set(!asc);

                let tbody_elem: Element = match doc2.get_element_by_id(&tbody) {
                    Some(el) => el,
                    None => return,
                };

                let rows: NodeList = match tbody_elem.query_selector_all("[data-data-table-row]") {
                    Ok(r) => r,
                    Err(_) => return,
                };

                let mut row_vec: Vec<Element> = vec![];
                let length: u32 = rows.length();

                for i in 0..length {
                    let node: Node = match rows.item(i) {
                        Some(n) => n,
                        None => continue,
                    };

                    let elem: &Element = match node.dyn_ref::<Element>() {
                        Some(e) => e,
                        None => continue,
                    };

                    row_vec.push(elem.clone());
                }

                row_vec.sort_by(|a: &Element, b: &Element| {
                    let a_val: String = a.get_attribute("data-product").unwrap_or_default();
                    let b_val: String = b.get_attribute("data-product").unwrap_or_default();
                    if asc {
                        a_val.cmp(&b_val)
                    } else {
                        b_val.cmp(&a_val)
                    }
                });

                for row in row_vec {
                    let _ = tbody_elem.append_child(&row);
                }
            }) as Box<dyn FnMut(_)>);

            let target2: &EventTarget = sort_btn.as_ref();
            let _ = target2.add_event_listener_with_callback("click", sort_closure.as_ref().unchecked_ref());
            sort_closure.forget();

            // Row actions
            let buttons: NodeList = match document.query_selector_all("[data-action]") {
                Ok(b) => b,
                Err(_) => return,
            };

            let length: u32 = buttons.length();
            for i in 0..length {
                let node: Node = match buttons.item(i) {
                    Some(n) => n,
                    None => continue,
                };

                let doc3: Document = document.clone();

                let action_closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    let target_et: EventTarget = match e.current_target() {
                        Some(t) => t,
                        None => return,
                    };

                    let element: Element = match target_et.dyn_into() {
                        Ok(el) => el,
                        Err(_) => return,
                    };

                    let action: String = element.get_attribute("data-action").unwrap_or_default();
                    let row_id: String = element.get_attribute("data-row").unwrap_or_default();

                    let row: Element = match doc3.query_selector(&format!("[data-row-id='{}']", row_id)) {
                        Ok(Some(r)) => r,
                        _ => return,
                    };

                    let w = window();
                    match action.as_str() {
                        "view-product" => {
                            let product: String = row.get_attribute("data-product").unwrap_or_default();
                            let _ = w.alert_with_message(&format!("Product: {}", product));
                        }
                        "view-price" => {
                            let price: String = row.get_attribute("data-price").unwrap_or_default();
                            let _ = w.alert_with_message(&format!("Price: ${}", price));
                        }
                        "copy-row" => {
                            let product: String = row.get_attribute("data-product").unwrap_or_default();
                            let price: String = row.get_attribute("data-price").unwrap_or_default();
                            let stock: String = row.get_attribute("data-stock").unwrap_or_default();
                            let text: String = format!("{} | ${} | {} units", product, price, stock);
                            let _ = w.alert_with_message(&format!("Copied: {}", text));
                        }
                        _ => {}
                    }
                }) as Box<dyn FnMut(_)>);

                let target3: &EventTarget = node.as_ref();
                let _ = target3.add_event_listener_with_callback("click", action_closure.as_ref().unchecked_ref());
                action_closure.forget();
            }
        });
    }
}
