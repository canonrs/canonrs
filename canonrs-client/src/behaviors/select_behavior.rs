#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut s = el.get_attribute("data-rs-state").unwrap_or_default();
    if !s.split_whitespace().any(|x| x == state) {
        s = format!("{} {}", s, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &s).ok();
}

#[cfg(feature = "hydrate")]
fn remove_state(el: &web_sys::Element, state: &str) {
    if let Some(s) = el.get_attribute("data-rs-state") {
        let f = s.split_whitespace().filter(|x| *x != state).collect::<Vec<_>>().join(" ");
        if f.is_empty() { el.remove_attribute("data-rs-state").ok(); }
        else { el.set_attribute("data-rs-state", &f).ok(); }
    }
}

#[cfg(feature = "hydrate")]
fn has_state(el: &web_sys::Element, state: &str) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.split_whitespace().any(|x| x == state)).unwrap_or(false)
}

#[cfg(feature = "hydrate")]
fn get_navigable_items(root: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let mut items = vec![];
    if let Ok(nodes) = root.query_selector_all("[data-rs-select-item]") {
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                    if !el.has_attribute("data-rs-disabled") { items.push(el); }
                }
            }
        }
    }
    items
}

#[cfg(feature = "hydrate")]
fn scroll_item_into_list(root: &web_sys::Element, item: &web_sys::HtmlElement) {
    if let Ok(Some(list)) = root.query_selector("[data-rs-select-content]") {
        if let Ok(list_el) = list.dyn_into::<web_sys::HtmlElement>() {
            let item_top = item.offset_top();
            let item_h   = item.offset_height();
            let list_scroll = list_el.scroll_top();
            let list_h   = list_el.offset_height();
            if item_top < list_scroll {
                list_el.set_scroll_top(item_top);
            } else if item_top + item_h > list_scroll + list_h {
                list_el.set_scroll_top(item_top + item_h - list_h);
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn close_select(root: &web_sys::Element) {
    root.set_attribute("data-rs-state", "closed").ok();
    // clear focus/hover on items
    if let Ok(items) = root.query_selector_all("[data-rs-select-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    remove_state(&el, "focus");
                    remove_state(&el, "hover");
                }
            }
        }
    }
    if let Ok(Some(t)) = root.query_selector("[data-rs-select-trigger]") {
        t.set_attribute("aria-expanded", "false").ok();
    }
}

#[cfg(feature = "hydrate")]
fn select_item(root: &web_sys::Element, target: &web_sys::Element) {
    let text = target.text_content().unwrap_or_default();
    let value = target.get_attribute("data-rs-value").unwrap_or_default();
    // clear all
    if let Ok(all) = root.query_selector_all("[data-rs-select-item]") {
        for j in 0..all.length() {
            if let Some(n) = all.item(j) {
                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                    remove_state(&el, "selected");
                    el.set_attribute("aria-selected", "false").ok();
                }
            }
        }
    }
    add_state(target, "selected");
    target.set_attribute("aria-selected", "true").ok();
    // update trigger display
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-select-trigger]") {
        if let Ok(Some(val_el)) = trigger.query_selector("[data-rs-select-value]") {
            val_el.set_text_content(Some(&text));
        }
    }
    root.set_attribute("data-rs-value", &value).ok();
    close_select(root);
    if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
        root.dispatch_event(&event).ok();
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-select", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-select-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-select-attached", "1").ok();

        // sync disabled
        let trigger_disabled = root.query_selector("[data-rs-select-trigger][data-rs-disabled]").ok().flatten().is_some();
        if root.has_attribute("data-rs-disabled") || trigger_disabled {
            if let Ok(Some(t)) = root.query_selector("[data-rs-select-trigger]") {
                add_state(&t, "disabled");
            }
            add_state(root, "disabled");
            return Ok(());
        }
        if root.get_attribute("data-rs-state").is_none() {
            root.set_attribute("data-rs-state", "closed").ok();
        }

        // ARIA: listbox id + aria-controls no trigger
        if let Ok(Some(content)) = root.query_selector("[data-rs-select-content]") {
            let list_id = content.get_attribute("id").unwrap_or_else(|| {
                let id = format!("sel-list-{}", js_sys::Math::random().to_string().replace(".", ""));
                content.set_attribute("id", &id).ok();
                id
            });
            if let Ok(Some(t)) = root.query_selector("[data-rs-select-trigger]") {
                t.set_attribute("aria-controls", &list_id).ok();
            }
        }

        // sync disabled on items
        if let Ok(items) = root.query_selector_all("[data-rs-select-item][data-rs-disabled]") {
            for i in 0..items.length() {
                if let Some(node) = items.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() { add_state(&el, "disabled"); }
                }
            }
        }

        let Ok(Some(trigger)) = root.query_selector("[data-rs-select-trigger]") else { return Ok(()); };

        // toggle open/close on trigger click
        { let r = root.clone(); let t = trigger.clone();
          let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
              e.stop_propagation();
              if has_state(&r, "open") {
                  close_select(&r);
              } else {
                  r.set_attribute("data-rs-state", "open").ok();
                  t.set_attribute("aria-expanded", "true").ok();
                  // focus primeiro item nao disabled
                  let items = get_navigable_items(&r);
                  // se ja tem selected, foca nele
                  let focus_target = items.iter().find(|el| has_state(el, "selected"))
                      .or_else(|| items.first());
                  if let Some(el) = focus_target {
                      add_state(el, "focus");
                      scroll_item_into_list(&r, el);
                  }
              }
          }) as Box<dyn FnMut(_)>);
          trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // keyboard nav no trigger
        { let r = root.clone();
          let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
              let key = e.key();
              let is_open = has_state(&r, "open");
              match key.as_str() {
                  "Escape" | "Tab" => { close_select(&r); }
                  " " | "Enter" => {
                      e.prevent_default();
                      if !is_open {
                          r.set_attribute("data-rs-state", "open").ok();
                          if let Ok(Some(t)) = r.query_selector("[data-rs-select-trigger]") {
                              t.set_attribute("aria-expanded", "true").ok();
                          }
                      } else {
                          let items = get_navigable_items(&r);
                          if let Some(focused) = items.iter().find(|el| has_state(el, "focus")) {
                              let focused_el: &web_sys::Element = focused;
                              select_item(&r, focused_el);
                          }
                      }
                  }
                  "ArrowDown" | "ArrowUp" => {
                      e.prevent_default();
                      if !is_open {
                          r.set_attribute("data-rs-state", "open").ok();
                          if let Ok(Some(t)) = r.query_selector("[data-rs-select-trigger]") {
                              t.set_attribute("aria-expanded", "true").ok();
                          }
                      }
                      let items = get_navigable_items(&r);
                      let len = items.len();
                      if len == 0 { return; }
                      let focused_idx = items.iter().position(|el| has_state(el, "focus"));
                      for el in &items { remove_state(el, "focus"); }
                      let next = match (key.as_str(), focused_idx) {
                          ("ArrowDown", None)    => 0,
                          ("ArrowDown", Some(i)) => (i + 1).min(len - 1),
                          ("ArrowUp",   None)    => len - 1,
                          ("ArrowUp",   Some(i)) => if i == 0 { 0 } else { i - 1 },
                          _ => 0,
                      };
                      add_state(&items[next], "focus");
                      scroll_item_into_list(&r, &items[next]);
                  }
                  _ => {}
              }
          }) as Box<dyn FnMut(_)>);
          trigger.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // item click + hover
        if let Ok(items) = root.query_selector_all("[data-rs-select-item]") {
            for i in 0..items.length() {
                let Some(node) = items.item(i) else { continue };
                let Ok(item) = node.dyn_into::<web_sys::Element>() else { continue };
                if item.has_attribute("data-rs-disabled") { continue; }
                let r = root.clone();
                { let it = item.clone();
                  let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                      e.stop_propagation();
                      select_item(&r, &it);
                  }) as Box<dyn FnMut(_)>);
                  item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                  cb.forget(); }
                { let it = item.clone();
                  let cb = Closure::wrap(Box::new(move |_: MouseEvent| { add_state(&it, "hover"); }) as Box<dyn FnMut(_)>);
                  item.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref()).ok();
                  cb.forget(); }
                { let it = item.clone();
                  let cb = Closure::wrap(Box::new(move |_: MouseEvent| { remove_state(&it, "hover"); }) as Box<dyn FnMut(_)>);
                  item.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref()).ok();
                  cb.forget(); }
            }
        }

        // close outside
        { let r = root.clone();
          let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
              if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()) {
                  if !r.contains(Some(&target)) && has_state(&r, "open") {
                      close_select(&r);
                  }
              }
          }) as Box<dyn FnMut(_)>);
          web_sys::window().unwrap().document().unwrap()
              .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // focus state no trigger
        { let t = trigger.clone();
          let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| { add_state(&t, "focus"); }) as Box<dyn FnMut(_)>);
          trigger.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }
        { let t = trigger.clone();
          let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| { remove_state(&t, "focus"); }) as Box<dyn FnMut(_)>);
          trigger.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
