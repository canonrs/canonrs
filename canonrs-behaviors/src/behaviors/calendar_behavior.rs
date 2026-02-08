#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node, NodeList, Element, EventTarget};

pub struct CalendarController {
    root: HtmlElement,
    on_select: Callback<String>,
    on_month_change: Option<Callback<(i32, u32)>>,
}

impl CalendarController {
    pub fn new(root: HtmlElement, on_select: Callback<String>) -> Self {
        Self {
            root,
            on_select,
            on_month_change: None,
        }
    }

    pub fn with_month_change(mut self, on_month_change: Callback<(i32, u32)>) -> Self {
        self.on_month_change = Some(on_month_change);
        self
    }

    pub fn attach(&self) {
        let root = self.root.clone();
        let on_select = self.on_select;

        let root_kb = root.clone();
        let kb_handler = Closure::wrap(Box::new(move |ev: leptos::web_sys::KeyboardEvent| {
            let target_et: EventTarget = match ev.target() {
                Some(t) => t,
                None => return,
            };
            let current: HtmlElement = match target_et.dyn_into() {
                Ok(el) => el,
                Err(_) => return,
            };

            if current.get_attribute("data-calendar-cell").is_none() {
                return;
            }

            ev.stop_propagation();

            let cells: NodeList = match root_kb.query_selector_all("[data-calendar-cell]:not([data-disabled])") {
                Ok(list) => list,
                Err(_) => return,
            };

            let cells: Vec<HtmlElement> = (0..cells.length())
                .filter_map(|i| {
                    let node: Node = cells.item(i)?;
                    node.dyn_into::<HtmlElement>().ok()
                })
                .collect();

            if !cells.iter().any(|c: &HtmlElement| c.get_attribute("tabindex").as_deref() == Some("0")) {
                if let Some(first) = cells.first() {
                    let first_el: &HtmlElement = first;
                    first_el.set_attribute("tabindex", "0").ok();
                }
            }

            let Some(idx) = cells.iter().position(|c| c == &current) else { return };

            let cols: usize = 7;
            let len: usize = cells.len();

            let next: Option<usize> = match ev.key().as_str() {
                "ArrowLeft"  => if idx > 0 { Some(idx - 1) } else { None },
                "ArrowRight" => Some(if idx + 1 < len { idx + 1 } else { len - 1 }),
                "ArrowUp"    => if idx >= cols { Some(idx - cols) } else { None },
                "ArrowDown"  => Some(if idx + cols < len { idx + cols } else { len - 1 }),
                "Home"       => Some(0),
                "End"        => Some(len - 1),
                "PageUp"     => if idx >= cols * 4 { Some(idx - cols * 4) } else { Some(0) },
                "PageDown"   => Some(if idx + cols * 4 < len { idx + cols * 4 } else { len - 1 }),
                "Enter" | " " => {
                    ev.prevent_default();
                    if let Some(date) = current.get_attribute("data-date") {
                        let all_cells: NodeList = match root_kb.query_selector_all("[data-calendar-cell]") {
                            Ok(list) => list,
                            Err(_) => return,
                        };
                        let length: u32 = all_cells.length();
                        for i in 0..length {
                            let node: Node = match all_cells.item(i) {
                                Some(n) => n,
                                None => continue,
                            };
                            let cell_ref: &HtmlElement = match node.dyn_ref() {
                                Some(el) => el,
                                None => continue,
                            };
                            cell_ref.remove_attribute("data-selected").ok();
                        }
                        current.set_attribute("data-selected", "true").ok();
                        on_select.run(date);
                    }
                    return;
                }
                _ => return,
            };

            if let Some(n) = next {
                ev.prevent_default();

                for c in &cells {
                    let cell_item: &HtmlElement = c;
                    cell_item.set_attribute("tabindex", "-1").ok();
                }

                let cell = &cells[n];
                cell.set_attribute("tabindex", "0").ok();
                cell.focus().ok();
            }
        }) as Box<dyn FnMut(_)>);

        self.root.add_event_listener_with_callback("keydown", kb_handler.as_ref().unchecked_ref()).ok();
        kb_handler.forget();

        let root_click = root.clone();
        let root_click_inner = root.clone();
        let on_select_clone = self.on_select;

        let click_handler = Closure::wrap(Box::new(move |ev: leptos::web_sys::MouseEvent| {
            let target_et: EventTarget = match ev.target() {
                Some(t) => t,
                None => return,
            };
            let cell_html: HtmlElement = match target_et.dyn_into() {
                Ok(el) => el,
                Err(_) => return,
            };

            if cell_html.get_attribute("data-calendar-cell").is_none() {
                return;
            }

            if cell_html.get_attribute("data-disabled").is_some() {
                return;
            }

            if let Some(date) = cell_html.get_attribute("data-date") {
                let all_cells: NodeList = match root_click_inner.query_selector_all("[data-calendar-cell]") {
                    Ok(list) => list,
                    Err(_) => return,
                };
                let length: u32 = all_cells.length();
                for i in 0..length {
                    let node: Node = match all_cells.item(i) {
                        Some(n) => n,
                        None => continue,
                    };
                    let cell_ref: &HtmlElement = match node.dyn_ref() {
                        Some(el) => el,
                        None => continue,
                    };
                    cell_ref.remove_attribute("data-selected").ok();
                }
                cell_html.set_attribute("data-selected", "true").ok();
                on_select_clone.run(date);
            }
        }) as Box<dyn FnMut(_)>);

        root_click.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref()).ok();
        click_handler.forget();

        if let Some(on_month_change) = self.on_month_change {
            let root_nav = root.clone();

            let nav_handler = Closure::wrap(Box::new(move |ev: leptos::web_sys::MouseEvent| {
                let target_et: EventTarget = match ev.target() {
                    Some(t) => t,
                    None => return,
                };
                let btn_html: HtmlElement = match target_et.dyn_into() {
                    Ok(el) => el,
                    Err(_) => return,
                };

                let nav_dir = btn_html.get_attribute("data-calendar-nav");
                if nav_dir.is_none() {
                    return;
                }

                if let Ok(Some(title)) = root_nav.query_selector("[data-calendar-title]") {
                    let title_elem: Element = title;
                    let title_ref: &HtmlElement = match title_elem.dyn_ref() {
                        Some(el) => el,
                        None => return,
                    };
                    let text = title_ref.inner_text();
                    let parts: Vec<&str> = text.split_whitespace().collect();
                    if parts.len() == 2 {
                        let year: i32 = parts[1].parse().unwrap_or(2026);
                        let month_name = parts[0];

                        let month = match month_name {
                            "January" => 1, "February" => 2, "March" => 3,
                            "April" => 4, "May" => 5, "June" => 6,
                            "July" => 7, "August" => 8, "September" => 9,
                            "October" => 10, "November" => 11, "December" => 12,
                            _ => 1,
                        };

                        let (new_year, new_month) = match nav_dir.as_deref() {
                            Some("prev") => {
                                if month == 1 { (year - 1, 12) } else { (year, month - 1) }
                            },
                            Some("next") => {
                                if month == 12 { (year + 1, 1) } else { (year, month + 1) }
                            },
                            _ => return,
                        };

                        on_month_change.run((new_year, new_month));
                    }
                }
            }) as Box<dyn FnMut(_)>);

            self.root.add_event_listener_with_callback("click", nav_handler.as_ref().unchecked_ref()).ok();
            nav_handler.forget();
        }
    }
}
