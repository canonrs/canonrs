#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;

#[cfg(feature = "hydrate")]
struct DataTableState {
    all_rows: Vec<leptos::web_sys::Element>,
    current_page: usize,
    page_size: usize,
    filter_query: String,
    sort_column: Option<String>,
    sort_ascending: bool,
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-datatable", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let doc = document();
        if let Some(datatable_el) = doc.get_element_by_id(element_id) {
            
            // Capture original rows ONCE
            let tbody = datatable_el.query_selector("[data-datatable-body]").ok().flatten();
            let mut all_rows = Vec::new();
            
            if let Some(tbody_el) = tbody {
                let rows = tbody_el.query_selector_all("[data-datatable-row]").ok();
                if let Some(rows_list) = rows {
                    for i in 0..rows_list.length() {
                        if let Some(row) = rows_list.get(i) {
                            all_rows.push(row.unchecked_into());
                        }
                    }
                }
            }
            
            let state = Rc::new(RefCell::new(DataTableState {
                all_rows,
                current_page: 1,
                page_size: 5,
                filter_query: String::new(),
                sort_column: None,
                sort_ascending: true,
            }));
            
            setup_sort(&datatable_el, state.clone());
            setup_search(&datatable_el, state.clone());
            setup_pagination_buttons(&datatable_el, state.clone());
            
            render_table(&datatable_el, &state.borrow());
        }
        
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_sort(datatable: &leptos::web_sys::Element, state: Rc<RefCell<DataTableState>>) {
    use leptos::web_sys::Element;
    
    let headers = datatable.query_selector_all("[data-datatable-head-cell][data-sort-key]").ok();
    if let Some(headers_list) = headers {
        for i in 0..headers_list.length() {
            if let Some(header) = headers_list.get(i) {
                let header_el: Element = header.unchecked_into();
                let sort_key = header_el.get_attribute("data-sort-key").unwrap_or_default();
                
                if !sort_key.is_empty() {
                    let datatable_clone = datatable.clone();
                    let state_clone = state.clone();
                    let key = sort_key.clone();
                    
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: leptos::web_sys::Event| {
                        let mut s = state_clone.borrow_mut();
                        
                        if s.sort_column.as_ref() == Some(&key) {
                            s.sort_ascending = !s.sort_ascending;
                        } else {
                            s.sort_column = Some(key.clone());
                            s.sort_ascending = true;
                        }
                        
                        drop(s);
                        render_table(&datatable_clone, &state_clone.borrow());
                    }) as Box<dyn FnMut(_)>);
                    
                    header_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
                    closure.forget();
                    
                    let doc = document();
                    let icon = doc.create_element("span").ok();
                    if let Some(icon_el) = icon {
                        icon_el.set_attribute("class", "sort-icon").ok();
                        icon_el.set_inner_html(" ⇅");
                        header_el.append_child(&icon_el).ok();
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn setup_search(datatable: &leptos::web_sys::Element, state: Rc<RefCell<DataTableState>>) {
    let search_input = datatable.query_selector("[data-datatable-search]").ok().flatten();
    
    if let Some(input_el) = search_input {
        let datatable_clone = datatable.clone();
        let state_clone = state.clone();
        
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
            let target = e.target().unwrap();
            let input: &leptos::web_sys::HtmlInputElement = target.unchecked_ref();
            let value = input.value();
            
            state_clone.borrow_mut().filter_query = value;
            state_clone.borrow_mut().current_page = 1;
            
            render_table(&datatable_clone, &state_clone.borrow());
        }) as Box<dyn FnMut(_)>);
        
        input_el.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}

#[cfg(feature = "hydrate")]
fn setup_pagination_buttons(datatable: &leptos::web_sys::Element, state: Rc<RefCell<DataTableState>>) {
    let prev_btn = datatable.query_selector("[data-datatable-prev]").ok().flatten();
    let next_btn = datatable.query_selector("[data-datatable-next]").ok().flatten();
    
    if let Some(prev_el) = prev_btn {
        let datatable_clone = datatable.clone();
        let state_clone = state.clone();
        
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: leptos::web_sys::Event| {
            let mut s = state_clone.borrow_mut();
            if s.current_page > 1 {
                s.current_page -= 1;
            }
            drop(s);
            render_table(&datatable_clone, &state_clone.borrow());
        }) as Box<dyn FnMut(_)>);
        
        prev_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
    
    if let Some(next_el) = next_btn {
        let datatable_clone = datatable.clone();
        let state_clone = state.clone();
        
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: leptos::web_sys::Event| {
            let s = state_clone.borrow();
            let filtered = compute_filtered(&s);
            let total_pages = ((filtered.len() as f32) / (s.page_size as f32)).ceil() as usize;
            
            drop(s);
            
            let mut s = state_clone.borrow_mut();
            if s.current_page < total_pages {
                s.current_page += 1;
            }
            drop(s);
            
            render_table(&datatable_clone, &state_clone.borrow());
        }) as Box<dyn FnMut(_)>);
        
        next_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}

#[cfg(feature = "hydrate")]
fn render_table(datatable: &leptos::web_sys::Element, state: &DataTableState) {
    let tbody = datatable.query_selector("[data-datatable-body]").ok().flatten();
    if let Some(tbody_el) = tbody {
        // Step 1: Filter
        let filtered = compute_filtered(state);
        
        // Step 2: Sort
        let sorted = compute_sorted(datatable, &filtered, state);
        
        // Step 3: Paginate
        let total = sorted.len();
        let start = (state.current_page - 1) * state.page_size;
        let end = (start + state.page_size).min(total);
        let paginated: Vec<_> = sorted.into_iter().skip(start).take(end - start).collect();
        
        // Step 4: Render (single source of truth)
        // First, hide ALL rows
        for row in &state.all_rows {
            let style = row.dyn_ref::<leptos::web_sys::HtmlElement>().unwrap();
            style.style().set_property("display", "none").ok();
        }
        
        // Then, show and reorder only paginated rows
        for row in &paginated {
            let style = row.dyn_ref::<leptos::web_sys::HtmlElement>().unwrap();
            style.style().set_property("display", "").ok();
            tbody_el.append_child(row).ok();
        }
        
        // Update UI
        update_pagination_info(datatable, start + 1, end, total);
        update_sort_icons(datatable, state);
    }
}

#[cfg(feature = "hydrate")]
fn compute_filtered(state: &DataTableState) -> Vec<leptos::web_sys::Element> {
    state.all_rows.iter()
        .filter(|row| {
            let text = row.text_content().unwrap_or_default();
            state.filter_query.is_empty() || 
            text.to_lowercase().contains(&state.filter_query.to_lowercase())
        })
        .cloned()
        .collect()
}

#[cfg(feature = "hydrate")]
fn compute_sorted(datatable: &leptos::web_sys::Element, rows: &[leptos::web_sys::Element], state: &DataTableState) -> Vec<leptos::web_sys::Element> {
    let mut sorted = rows.to_vec();
    
    if let Some(ref sort_col) = state.sort_column {
        let col_idx = get_column_index(datatable, sort_col);
        sorted.sort_by(|a, b| {
            let val_a = get_cell_text(a, col_idx);
            let val_b = get_cell_text(b, col_idx);
            if state.sort_ascending { val_a.cmp(&val_b) } else { val_b.cmp(&val_a) }
        });
    }
    
    sorted
}

#[cfg(feature = "hydrate")]
fn get_column_index(datatable: &leptos::web_sys::Element, sort_key: &str) -> usize {
    use leptos::web_sys::Element;
    
    let headers = datatable.query_selector_all("[data-datatable-head-cell]").ok();
    if let Some(headers_list) = headers {
        for i in 0..headers_list.length() {
            if let Some(header) = headers_list.get(i) {
                let header_el: Element = header.unchecked_into();
                if header_el.get_attribute("data-sort-key").as_deref() == Some(sort_key) {
                    return i as usize;
                }
            }
        }
    }
    0
}

#[cfg(feature = "hydrate")]
fn get_cell_text(row: &leptos::web_sys::Element, col_idx: usize) -> String {
    let cells = row.query_selector_all("[data-datatable-cell]").ok();
    if let Some(cells_list) = cells {
        if let Some(cell) = cells_list.get(col_idx as u32) {
            return cell.text_content().unwrap_or_default().trim().to_lowercase();
        }
    }
    String::new()
}

#[cfg(feature = "hydrate")]
fn update_pagination_info(datatable: &leptos::web_sys::Element, start: usize, end: usize, total: usize) {
    let info = datatable.query_selector("[data-datatable-info]").ok().flatten();
    if let Some(info_el) = info {
        let s = if total == 0 { 0 } else { start };
        info_el.set_inner_html(&format!("Showing {}-{} of {}", s, end, total));
    }
}

#[cfg(feature = "hydrate")]
fn update_sort_icons(datatable: &leptos::web_sys::Element, state: &DataTableState) {
    use leptos::web_sys::Element;
    
    let headers = datatable.query_selector_all("[data-datatable-head-cell][data-sort-key]").ok();
    if let Some(headers_list) = headers {
        for i in 0..headers_list.length() {
            if let Some(header) = headers_list.get(i) {
                let header_el: Element = header.unchecked_into();
                let sort_key = header_el.get_attribute("data-sort-key").unwrap_or_default();
                
                if let Some(icon) = header_el.query_selector(".sort-icon").ok().flatten() {
                    let icon_text = if Some(sort_key.as_str()) == state.sort_column.as_deref() {
                        if state.sort_ascending { " ▲" } else { " ▼" }
                    } else {
                        " ⇅"
                    };
                    icon.set_inner_html(icon_text);
                }
                
                let aria = if Some(sort_key.as_str()) == state.sort_column.as_deref() {
                    if state.sort_ascending { "ascending" } else { "descending" }
                } else {
                    "none"
                };
                header_el.set_attribute("aria-sort", aria).ok();
            }
        }
    }
}
