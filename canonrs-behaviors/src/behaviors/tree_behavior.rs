#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-tree", Box::new(|element_id, _state| {
        let Some(tree) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        setup_tree(&tree)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_tree(tree: &Element) -> BehaviorResult<()> {
    if tree.get_attribute("data-tree-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = tree.set_attribute("data-tree-attached", "1");

    // Setup all tree items
    let items = tree.query_selector_all("[data-tree-item]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query items".into() })?;

    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            if let Ok(item) = node.dyn_into::<Element>() {
                setup_tree_item(&item)?;
            }
        }
    }

    // Keyboard navigation on tree container
    let tree_clone = tree.clone();
    let on_keydown = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        handle_tree_keyboard(&tree_clone, &e);
    }) as Box<dyn FnMut(_)>);

    tree.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "keydown".into() })?;
    on_keydown.forget();

    Ok(())
}

#[cfg(feature = "hydrate")]
fn setup_tree_item(item: &Element) -> BehaviorResult<()> {
    // Click to select
    let item_clone = item.clone();
    let on_click = Closure::wrap(Box::new(move |e: web_sys::Event| {
        e.stop_propagation();
        select_item(&item_clone);
    }) as Box<dyn FnMut(_)>);

    item.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "click".into() })?;
    on_click.forget();

    // Setup toggle button if has children
    if item.get_attribute("data-has-children").as_deref() == Some("true") {
        if let Some(toggle) = item.query_selector("[data-tree-toggle]").ok().flatten() {
            let item_clone = item.clone();
            let on_toggle = Closure::wrap(Box::new(move |e: web_sys::Event| {
                e.stop_propagation();
                toggle_expand(&item_clone);
            }) as Box<dyn FnMut(_)>);

            toggle.add_event_listener_with_callback("click", on_toggle.as_ref().unchecked_ref()).ok();
            on_toggle.forget();
        }
    }

    Ok(())
}

#[cfg(feature = "hydrate")]
fn handle_tree_keyboard(tree: &Element, e: &web_sys::KeyboardEvent) {
    let key = e.key();
    
    match key.as_str() {
        "ArrowDown" => {
            e.prevent_default();
            move_focus_down(tree);
        },
        "ArrowUp" => {
            e.prevent_default();
            move_focus_up(tree);
        },
        "ArrowRight" => {
            e.prevent_default();
            expand_focused_item(tree);
        },
        "ArrowLeft" => {
            e.prevent_default();
            collapse_focused_item(tree);
        },
        "Home" => {
            e.prevent_default();
            focus_first_item(tree);
        },
        "End" => {
            e.prevent_default();
            focus_last_visible_item(tree);
        },
        "Enter" | " " => {
            e.prevent_default();
            select_focused_item(tree);
        },
        _ => {}
    }
}

#[cfg(feature = "hydrate")]
fn get_visible_items(tree: &Element) -> Vec<Element> {
    let mut visible = Vec::new();
    
    if let Ok(all_items) = tree.query_selector_all("[data-tree-item]") {
        for i in 0..all_items.length() {
            if let Some(node) = all_items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    // Item is visible if it's not inside a collapsed parent
                    if is_item_visible(&item) {
                        visible.push(item);
                    }
                }
            }
        }
    }
    
    visible
}

#[cfg(feature = "hydrate")]
fn is_item_visible(item: &Element) -> bool {
    let mut current = item.parent_element();
    
    while let Some(parent) = current {
        if parent.has_attribute("data-tree-group") {
            // Check if parent item is expanded
            if let Some(parent_item) = parent.previous_element_sibling() {
                if parent_item.has_attribute("data-tree-item") {
                    if parent_item.get_attribute("data-expanded").as_deref() != Some("true") {
                        return false;
                    }
                }
            }
        }
        current = parent.parent_element();
    }
    
    true
}

#[cfg(feature = "hydrate")]
fn get_focused_item(tree: &Element) -> Option<Element> {
    tree.query_selector("[data-tree-item][tabindex='0']")
        .ok()
        .flatten()
}

#[cfg(feature = "hydrate")]
fn set_roving_tabindex(tree: &Element, new_focused: &Element) {
    // Remove tabindex=0 from all items
    if let Ok(all_items) = tree.query_selector_all("[data-tree-item]") {
        for i in 0..all_items.length() {
            if let Some(node) = all_items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    item.set_attribute("tabindex", "-1").ok();
                }
            }
        }
    }
    
    // Set tabindex=0 on new focused item
    new_focused.set_attribute("tabindex", "0").ok();
    
    if let Ok(html_el) = new_focused.clone().dyn_into::<web_sys::HtmlElement>() {
        html_el.focus().ok();
    }
}

#[cfg(feature = "hydrate")]
fn move_focus_down(tree: &Element) {
    let visible = get_visible_items(tree);
    if visible.is_empty() { return; }
    
    if let Some(current) = get_focused_item(tree) {
        if let Some(idx) = visible.iter().position(|el| el == &current) {
            if idx < visible.len() - 1 {
                set_roving_tabindex(tree, &visible[idx + 1]);
            }
        }
    } else {
        set_roving_tabindex(tree, &visible[0]);
    }
}

#[cfg(feature = "hydrate")]
fn move_focus_up(tree: &Element) {
    let visible = get_visible_items(tree);
    if visible.is_empty() { return; }
    
    if let Some(current) = get_focused_item(tree) {
        if let Some(idx) = visible.iter().position(|el| el == &current) {
            if idx > 0 {
                set_roving_tabindex(tree, &visible[idx - 1]);
            }
        }
    } else {
        set_roving_tabindex(tree, &visible[0]);
    }
}

#[cfg(feature = "hydrate")]
fn focus_first_item(tree: &Element) {
    let visible = get_visible_items(tree);
    if let Some(first) = visible.first() {
        set_roving_tabindex(tree, first);
    }
}

#[cfg(feature = "hydrate")]
fn focus_last_visible_item(tree: &Element) {
    let visible = get_visible_items(tree);
    if let Some(last) = visible.last() {
        set_roving_tabindex(tree, last);
    }
}

#[cfg(feature = "hydrate")]
fn expand_focused_item(tree: &Element) {
    if let Some(item) = get_focused_item(tree) {
        if item.get_attribute("data-has-children").as_deref() == Some("true") {
            if item.get_attribute("data-expanded").as_deref() != Some("true") {
                toggle_expand(&item);
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn collapse_focused_item(tree: &Element) {
    if let Some(item) = get_focused_item(tree) {
        if item.get_attribute("data-has-children").as_deref() == Some("true") {
            if item.get_attribute("data-expanded").as_deref() == Some("true") {
                toggle_expand(&item);
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn select_focused_item(tree: &Element) {
    if let Some(item) = get_focused_item(tree) {
        select_item(&item);
    }
}

#[cfg(feature = "hydrate")]
fn select_item(item: &Element) {
    // Get tree root
    let mut current = Some(item.clone());
    let mut tree: Option<Element> = None;
    
    while let Some(el) = current {
        if el.has_attribute("data-tree") {
            tree = Some(el);
            break;
        }
        current = el.parent_element();
    }
    
    let Some(tree_el) = tree else { return; };
    
    // Clear all selections
    if let Ok(all_items) = tree_el.query_selector_all("[data-tree-item]") {
        for i in 0..all_items.length() {
            if let Some(node) = all_items.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    el.set_attribute("data-selected", "false").ok();
                    el.set_attribute("aria-selected", "false").ok();
                }
            }
        }
    }
    
    // Select this item
    item.set_attribute("data-selected", "true").ok();
    item.set_attribute("aria-selected", "true").ok();
    
    // Emit event
    dispatch_select_event(item);
}

#[cfg(feature = "hydrate")]
fn toggle_expand(item: &Element) {
    let is_expanded = item.get_attribute("data-expanded").as_deref() == Some("true");
    let new_state = !is_expanded;
    
    item.set_attribute("data-expanded", &new_state.to_string()).ok();
    item.set_attribute("aria-expanded", &new_state.to_string()).ok();
    
    // Emit event
    dispatch_expand_event(item, new_state);
}

#[cfg(feature = "hydrate")]
fn dispatch_select_event(item: &Element) {
    let detail = js_sys::Object::new();
    if let Some(id) = item.get_attribute("id") {
        js_sys::Reflect::set(&detail, &JsValue::from_str("itemId"), &JsValue::from_str(&id)).ok();
    }

    let f = js_sys::Function::new_with_args(
        "el,detail",
        "el.dispatchEvent(new CustomEvent('canon:tree-select', {bubbles:true, detail:detail}));"
    );
    f.call2(&JsValue::NULL, item, &detail).ok();
}

#[cfg(feature = "hydrate")]
fn dispatch_expand_event(item: &Element, expanded: bool) {
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &JsValue::from_str("expanded"), &JsValue::from_bool(expanded)).ok();

    let f = js_sys::Function::new_with_args(
        "el,detail",
        "el.dispatchEvent(new CustomEvent('canon:tree-expand', {bubbles:true, detail:detail}));"
    );
    f.call2(&JsValue::NULL, item, &detail).ok();
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
