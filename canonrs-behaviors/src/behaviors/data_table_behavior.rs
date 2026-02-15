#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, closure::Closure};
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-datatable", Box::new(|element_id, _state| {
        let Some(container) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        setup_filter(&container)?;
        setup_sorting(&container)?;
        setup_pagination(&container)?;
        setup_column_toggle(&container)?;
        setup_selection(&container)?;
        setup_density(&container)?;
        setup_expand(&container)?;
        let all_rows = get_all_rows(&container);
        let page_size = container.get_attribute("data-page-size")
            .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(10);
        apply_pagination_vec(&all_rows, 1, page_size);
        update_pagination_ui(&container);
        Ok(())
    }));
}

// ─── HELPERS ──────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn get_all_rows(container: &Element) -> Vec<Element> {
    container.query_selector_all("[data-datatable-row]")
        .map(|nl| (0..nl.length()).filter_map(|i| nl.item(i)?.dyn_into::<Element>().ok()).collect())
        .unwrap_or_default()
}

#[cfg(feature = "hydrate")]
fn apply_pagination_vec(all_rows: &[Element], page: usize, page_size: usize) {
    let visible: Vec<&Element> = all_rows.iter()
        .filter(|el| el.get_attribute("data-filtered-hidden").is_none())
        .collect();

    let start = (page - 1) * page_size;
    let end = start + page_size;

    for (idx, row) in visible.iter().enumerate() {
        if idx >= start && idx < end {
            let _ = row.remove_attribute("hidden");
        } else {
            let _ = row.set_attribute("hidden", "");
        }
    }
}

#[cfg(feature = "hydrate")]
fn update_pagination_ui(container: &Element) {
    let current = container.get_attribute("data-current-page")
        .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(1);
    let total = container.get_attribute("data-total-pages")
        .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(1);

    if let Some(info) = container.query_selector("[data-pagination-info]").ok().flatten() {
        info.set_text_content(Some(&format!("{} of {}", current, total)));
    }

    let Ok(Some(pag)) = container.query_selector("[data-datatable-pagination]") else { return };
    let Ok(buttons) = pag.query_selector_all("button") else { return };

    for i in 0..buttons.length() {
        let Some(btn) = buttons.item(i) else { continue };
        let Ok(el) = btn.dyn_into::<Element>() else { continue };
        match el.get_attribute("data-action").as_deref() {
            Some("prev") => {
                if current <= 1 { let _ = el.set_attribute("disabled", ""); }
                else { let _ = el.remove_attribute("disabled"); }
            }
            Some("next") => {
                if current >= total { let _ = el.set_attribute("disabled", ""); }
                else { let _ = el.remove_attribute("disabled"); }
            }
            _ => {}
        }
    }
}

#[cfg(feature = "hydrate")]
fn get_cell_text(row: &Element, col_index: usize) -> String {
    row.query_selector_all("[data-datatable-cell]")
        .ok()
        .and_then(|nl| nl.item(col_index as u32))
        .and_then(|c| c.dyn_into::<Element>().ok())
        .and_then(|c| c.text_content())
        .unwrap_or_default()
        .trim()
        .to_lowercase()
}

// ─── FILTER ───────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_filter(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-filter-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-filter-attached", "1");

    let Some(input) = container.query_selector("[data-datatable-filter]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })? else {
        return Ok(());
    };

    let container_clone = container.clone();
    let input_clone = input.clone();

    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let query = input_clone
            .dyn_ref::<web_sys::HtmlInputElement>()
            .map(|el| el.value().to_lowercase())
            .unwrap_or_default();
        apply_filter(&container_clone, &query);
        update_pagination_ui(&container_clone);
    }) as Box<dyn FnMut(_)>);

    input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
    closure.forget();
    Ok(())
}

#[cfg(feature = "hydrate")]
fn apply_filter(container: &Element, query: &str) {
    let all_rows = get_all_rows(container);
    let empty = container.query_selector("[data-datatable-empty]").ok().flatten();
    let mut visible_count = 0usize;

    for row in &all_rows {
        let text = row.text_content().unwrap_or_default().to_lowercase();
        let matches = query.is_empty() || text.contains(query);
        if matches {
            let _ = row.remove_attribute("data-filtered-hidden");
            visible_count += 1;
        } else {
            let _ = row.set_attribute("data-filtered-hidden", "1");
            let _ = row.set_attribute("hidden", "");
        }
    }

    if let Some(empty_el) = empty {
        if visible_count == 0 { let _ = empty_el.remove_attribute("hidden"); }
        else { let _ = empty_el.set_attribute("hidden", ""); }
    }

    let page_size = container.get_attribute("data-page-size")
        .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(10);
    let total_pages = ((visible_count as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let _ = container.set_attribute("data-total-pages", &total_pages.to_string());
    let _ = container.set_attribute("data-current-page", "1");
    apply_pagination_vec(&all_rows, 1, page_size);
}

// ─── SORTING ──────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_sorting(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-sorting-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-sorting-attached", "1");

    let headers = container.query_selector_all("[data-datatable-head-cell][data-sort-key]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..headers.length() {
        let Some(header) = headers.item(i) else { continue };
        let Ok(header_el) = header.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let header_clone = header_el.clone();
        let all_headers = headers.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let current = header_clone.get_attribute("aria-sort").unwrap_or("none".into());
            let new_dir = match current.as_str() {
                "ascending" => "descending",
                "descending" => "none",
                _ => "ascending",
            };

            // Reset todos headers
            for j in 0..all_headers.length() {
                if let Some(h) = all_headers.item(j) {
                    if let Ok(el) = h.dyn_into::<Element>() {
                        let _ = el.set_attribute("aria-sort", "none");
                        if let Some(icon) = el.query_selector("[data-datatable-sort-icon]").ok().flatten() {
                            icon.set_text_content(Some("↕"));
                        }
                    }
                }
            }

            let _ = header_clone.set_attribute("aria-sort", new_dir);
            if let Some(icon) = header_clone.query_selector("[data-datatable-sort-icon]").ok().flatten() {
                icon.set_text_content(Some(match new_dir {
                    "ascending" => "▲",
                    "descending" => "▼",
                    _ => "↕",
                }));
            }

            // Remove hidden de todas as rows não filtradas antes de reordenar
            let all_rows = get_all_rows(&container_clone);
            for row in &all_rows {
                if row.get_attribute("data-filtered-hidden").is_none() {
                    let _ = row.remove_attribute("hidden");
                }
            }

            if new_dir != "none" {
                let col_index = header_clone.get_attribute("data-col-index")
                    .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(0);
                sort_rows(&container_clone, col_index, new_dir);
            }

            let page_size = container_clone.get_attribute("data-page-size")
                .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(10);
            let _ = container_clone.set_attribute("data-current-page", "1");
            apply_pagination_vec(&all_rows, 1, page_size);
            update_pagination_ui(&container_clone);
        }) as Box<dyn FnMut(_)>);

        header_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}

#[cfg(feature = "hydrate")]
fn sort_rows(container: &Element, col_index: usize, direction: &str) {
    let Ok(Some(tbody)) = container.query_selector("[data-datatable-body]") else { return };
    let Ok(rows) = tbody.query_selector_all("[data-datatable-row]") else { return };

    let mut row_vec: Vec<Element> = (0..rows.length())
        .filter_map(|i| rows.item(i)?.dyn_into::<Element>().ok())
        .collect();

    row_vec.sort_by(|a, b| {
        let cmp = get_cell_text(a, col_index).cmp(&get_cell_text(b, col_index));
        if direction == "descending" { cmp.reverse() } else { cmp }
    });

    for row in &row_vec {
        tbody.append_child(row).ok();
    }
}

// ─── PAGINATION ───────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_pagination(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-pagination-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-pagination-attached", "1");

    let Some(pag) = container.query_selector("[data-datatable-pagination]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })? else {
        return Ok(());
    };

    let buttons = pag.query_selector_all("button")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..buttons.length() {
        let Some(btn) = buttons.item(i) else { continue };
        let Ok(btn_el) = btn.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let action = btn_el.get_attribute("data-action").unwrap_or_default();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let current = container_clone.get_attribute("data-current-page")
                .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(1);
            let total = container_clone.get_attribute("data-total-pages")
                .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(1);

            let new_page = match action.as_str() {
                "prev" => current.saturating_sub(1).max(1),
                "next" => (current + 1).min(total),
                _ => current,
            };

            if new_page != current {
                let _ = container_clone.set_attribute("data-current-page", &new_page.to_string());
                let all_rows = get_all_rows(&container_clone);
                let page_size = container_clone.get_attribute("data-page-size")
                    .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(10);
                apply_pagination_vec(&all_rows, new_page, page_size);
                update_pagination_ui(&container_clone);
            }
        }) as Box<dyn FnMut(_)>);

        btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}

// ─── COLUMN TOGGLE ────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_column_toggle(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-column-toggle-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-column-toggle-attached", "1");

    let items = container
        .query_selector_all("[data-dropdown-menu-checkbox-item][data-col-index]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..items.length() {
        let Some(item) = items.item(i) else { continue };
        let Ok(item_el) = item.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let item_clone = item_el.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let col_index = item_clone.get_attribute("data-col-index")
                .and_then(|v: String| v.parse::<usize>().ok()).unwrap_or(0);
            let checked = item_clone.get_attribute("aria-checked").as_deref() == Some("true");
            let new_checked = !checked;
            let _ = item_clone.set_attribute("aria-checked", if new_checked { "true" } else { "false" });
            toggle_column(&container_clone, col_index, new_checked);
        }) as Box<dyn FnMut(_)>);

        item_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}

#[cfg(feature = "hydrate")]
fn toggle_column(container: &Element, col_index: usize, visible: bool) {
    let selector = format!("[data-col-index='{}']:not([data-dropdown-menu-checkbox-item])", col_index);
    let Ok(els) = container.query_selector_all(&selector) else { return };
    for i in 0..els.length() {
        if let Some(el) = els.item(i).and_then(|e| e.dyn_into::<Element>().ok()) {
            if visible { let _ = el.remove_attribute("hidden"); }
            else { let _ = el.set_attribute("hidden", ""); }
        }
    }
}


// ─── SELECTION ───────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_selection(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-selection-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-selection-attached", "1");

    // Select-all checkbox
    if let Some(select_all) = container.query_selector("[data-datatable-select-all]").ok().flatten() {
        let container_clone = container.clone();
        let sa_clone = select_all.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let checked = sa_clone.dyn_ref::<web_sys::HtmlInputElement>()
                .map(|el| el.checked()).unwrap_or(false);
            let rows = get_all_rows(&container_clone);
            for row in &rows {
                if row.get_attribute("data-filtered-hidden").is_some() { continue; }
                if checked {
                    let _ = row.set_attribute("data-state", "selected");
                } else {
                    let _ = row.remove_attribute("data-state");
                }
                if let Some(cb) = row.query_selector("[data-datatable-select-row]").ok().flatten() {
                    if let Some(input) = cb.dyn_ref::<web_sys::HtmlInputElement>() {
                        input.set_checked(checked);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        select_all.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }

    // Per-row checkboxes
    let row_cbs = container.query_selector_all("[data-datatable-select-row]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..row_cbs.length() {
        let Some(cb) = row_cbs.item(i) else { continue };
        let Ok(cb_el) = cb.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let cb_clone = cb_el.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let checked = cb_clone.dyn_ref::<web_sys::HtmlInputElement>()
                .map(|el| el.checked()).unwrap_or(false);

            // Find parent row
            let mut parent = cb_clone.parent_element();
            while let Some(p) = parent {
                if p.has_attribute("data-datatable-row") {
                    if checked {
                        let _ = p.set_attribute("data-state", "selected");
                    } else {
                        let _ = p.remove_attribute("data-state");
                    }
                    break;
                }
                parent = p.parent_element();
            }

            // Update select-all state
            update_select_all(&container_clone);
        }) as Box<dyn FnMut(_)>);

        cb_el.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}

#[cfg(feature = "hydrate")]
fn update_select_all(container: &Element) {
    let Some(select_all) = container.query_selector("[data-datatable-select-all]").ok().flatten() else { return };
    let Some(sa_input) = select_all.dyn_ref::<web_sys::HtmlInputElement>() else { return };

    let rows = get_all_rows(container);
    let visible: Vec<&Element> = rows.iter()
        .filter(|r| r.get_attribute("data-filtered-hidden").is_none())
        .collect();

    let selected = visible.iter().filter(|r| r.get_attribute("data-state").as_deref() == Some("selected")).count();

    if selected == 0 {
        sa_input.set_checked(false);
        sa_input.set_indeterminate(false);
    } else if selected == visible.len() {
        sa_input.set_checked(true);
        sa_input.set_indeterminate(false);
    } else {
        sa_input.set_checked(false);
        sa_input.set_indeterminate(true);
    }
}

// ─── DENSITY ─────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_density(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-density-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-density-attached", "1");

    let buttons = container.query_selector_all("[data-density-btn]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..buttons.length() {
        let Some(btn) = buttons.item(i) else { continue };
        let Ok(btn_el) = btn.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let btn_clone = btn_el.clone();
        let all_buttons = buttons.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let density = btn_clone.get_attribute("data-density-btn").unwrap_or_default();

            // Update container density
            let _ = container_clone.set_attribute("data-density", &density);

            // Update active state on buttons
            for j in 0..all_buttons.length() {
                if let Some(b) = all_buttons.item(j) {
                    if let Ok(el) = b.dyn_into::<Element>() {
                        let is_active = el.get_attribute("data-density-btn").as_deref() == Some(&density);
                        let _ = el.set_attribute("data-active", if is_active { "true" } else { "false" });
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}

// ─── EXPAND ───────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_expand(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-expand-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-expand-attached", "1");

    let buttons = container.query_selector_all("[data-datatable-expand-btn]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..buttons.length() {
        let Some(btn) = buttons.item(i) else { continue };
        let Ok(btn_el) = btn.dyn_into::<Element>() else { continue };
        let container_clone = container.clone();
        let btn_clone = btn_el.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let row_id = btn_clone.get_attribute("data-row-id").unwrap_or_default();
            let expanded = btn_clone.get_attribute("aria-expanded").as_deref() == Some("true");
            let new_expanded = !expanded;

            let _ = btn_clone.set_attribute("aria-expanded", if new_expanded { "true" } else { "false" });
            btn_clone.set_text_content(Some(if new_expanded { "▼" } else { "▶" }));

            let selector = format!("[data-datatable-expand-row][data-row-id='{}']", row_id);
            if let Ok(Some(expand_row)) = container_clone.query_selector(&selector) {
                if new_expanded {
                    let _ = expand_row.remove_attribute("hidden");
                } else {
                    let _ = expand_row.set_attribute("hidden", "");
                }
            }
        }) as Box<dyn FnMut(_)>);

        btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
        closure.forget();
    }
    Ok(())
}
#[cfg(not(feature = "hydrate"))]
pub fn register() {}
