#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, Element};

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut states = el.get_attribute("data-rs-state").unwrap_or_default();
    if !states.split_whitespace().any(|s| s == state) {
        states = format!("{} {}", states, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &states).ok();
}

#[cfg(feature = "hydrate")]
fn remove_state(el: &web_sys::Element, state: &str) {
    if let Some(states) = el.get_attribute("data-rs-state") {
        let f = states.split_whitespace().filter(|s| *s != state).collect::<Vec<_>>().join(" ");
        if f.is_empty() { el.remove_attribute("data-rs-state").ok(); }
        else { el.set_attribute("data-rs-state", &f).ok(); }
    }
}

#[cfg(feature = "hydrate")]
fn has_state(el: &web_sys::Element, state: &str) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.split_whitespace().any(|x| x == state)).unwrap_or(false)
}

#[cfg(feature = "hydrate")]
fn clear_hover_focus(root: &web_sys::Element) {
    if let Ok(items) = root.query_selector_all("[data-rs-combobox-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    remove_state(&el, "hover");
                    remove_state(&el, "focus");
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn get_navigable_items(root: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let mut items = vec![];
    if let Ok(nodes) = root.query_selector_all("[data-rs-combobox-item]") {
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                    let state = el.get_attribute("data-rs-state").unwrap_or_default();
                    let is_hidden = state.split_whitespace().any(|s| s == "hidden");
                    let is_disabled = el.has_attribute("data-rs-disabled");
                    if !is_hidden && !is_disabled { items.push(el); }
                }
            }
        }
    }
    items
}

#[cfg(feature = "hydrate")]
fn update_aria_activedescendant(root: &web_sys::Element, input: &web_sys::HtmlElement) {
    if let Ok(focused) = root.query_selector("[data-rs-combobox-item][data-rs-state~=\"focus\"]") {
        match focused {
            Some(el) => {
                let id = el.get_attribute("id").unwrap_or_else(|| {
                    let new_id = format!("cbx-item-{}", js_sys::Math::random().to_string().replace(".", ""));
                    el.set_attribute("id", &new_id).ok();
                    new_id
                });
                input.set_attribute("aria-activedescendant", &id).ok();
            }
            None => { input.remove_attribute("aria-activedescendant").ok(); }
        }
    }
}

#[cfg(feature = "hydrate")]
fn close_combobox(root: &web_sys::Element) {
    root.set_attribute("data-rs-state", "closed").ok();
    root.set_attribute("aria-expanded", "false").ok();
    clear_hover_focus(root);
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-combobox-trigger]") {
        trigger.set_attribute("aria-expanded", "false").ok();
    }
}

#[cfg(feature = "hydrate")]
fn open_combobox(root: &web_sys::Element) {
    root.set_attribute("data-rs-state", "open").ok();
    root.set_attribute("aria-expanded", "true").ok();
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-combobox-trigger]") {
        trigger.set_attribute("aria-expanded", "true").ok();
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-combobox", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-combobox-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-combobox-attached", "1").ok();
        if root.get_attribute("data-rs-state").is_none() {
            root.set_attribute("data-rs-state", "closed").ok();
        }

        // ARIA: role=combobox já no primitive; garantir listbox id
        if let Ok(Some(list)) = root.query_selector("[data-rs-combobox-list]") {
            let list_id = list.get_attribute("id").unwrap_or_else(|| {
                let id = format!("cbx-list-{}", js_sys::Math::random().to_string().replace(".", ""));
                list.set_attribute("id", &id).ok();
                id
            });
            if let Ok(Some(input_el)) = root.query_selector("[data-rs-combobox-input]") {
                input_el.set_attribute("aria-controls", &list_id).ok();
                input_el.set_attribute("role", "combobox").ok();
                input_el.set_attribute("aria-expanded", "false").ok();
            }
        }

        // sync disabled on items
        if let Ok(items) = root.query_selector_all("[data-rs-combobox-item][data-rs-disabled]") {
            for i in 0..items.length() {
                if let Some(node) = items.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        add_state(&el, "disabled");
                    }
                }
            }
        }

        let Ok(Some(input_el)) = root.query_selector("[data-rs-combobox-input]") else { return Ok(()); };
        let Ok(input) = input_el.dyn_into::<HtmlElement>() else { return Ok(()); };

        // open on focusin
        { let r = root.clone(); let inp = input.clone();
          let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
              open_combobox(&r);
              inp.set_attribute("aria-expanded", "true").ok();
          }) as Box<dyn FnMut(_)>);
          root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // filter on input
        { let r = root.clone();
          let cb = Closure::wrap(Box::new(move |e: web_sys::InputEvent| {
              let query = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                  .map(|el| el.value().to_lowercase()).unwrap_or_default();
              if let Ok(items) = r.query_selector_all("[data-rs-combobox-item]") {
                  for i in 0..items.length() {
                      if let Some(node) = items.item(i) {
                          if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                              let text = el.text_content().unwrap_or_default().to_lowercase();
                              let currently_disabled = el.has_attribute("data-rs-disabled");
                              if currently_disabled { continue; }
                              if query.is_empty() || text.contains(&query) {
                                  remove_state(&el, "hidden");
                              } else {
                                  add_state(&el, "hidden");
                                  remove_state(&el, "focus");
                              }
                          }
                      }
                  }
              }
          }) as Box<dyn FnMut(_)>);
          input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // close on focusout (Tab ou click fora)
        { let r = root.clone(); let inp = input.clone();
          let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
              let r2 = r.clone(); let inp2 = inp.clone();
              let closure = Closure::once(Box::new(move || {
                  let inside = web_sys::window().and_then(|w| w.document())
                      .and_then(|d| d.active_element())
                      .and_then(|el| el.closest("[data-rs-combobox]").ok().flatten()).is_some();
                  if !inside {
                      close_combobox(&r2);
                      inp2.set_attribute("aria-expanded", "false").ok();
                      inp2.remove_attribute("aria-activedescendant").ok();
                  }
              }) as Box<dyn FnOnce()>);
              web_sys::window().unwrap()
                  .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 150).ok();
              closure.forget();
          }) as Box<dyn FnMut(_)>);
          root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // click fora
        { let r = root.clone();
          let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
              if let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                  if !target.closest("[data-rs-combobox]").ok().flatten().is_some() {
                      close_combobox(&r);
                  }
              }
          }) as Box<dyn FnMut(_)>);
          web_sys::window().unwrap().document().unwrap()
              .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // keyboard: ArrowDown/Up/Enter/Escape/Tab
        { let r = root.clone(); let inp = input.clone();
          let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
              let key = e.key();
              let is_open = has_state(&r, "open");
              match key.as_str() {
                  "Escape" | "Tab" => {
                      close_combobox(&r);
                      inp.set_attribute("aria-expanded", "false").ok();
                      inp.remove_attribute("aria-activedescendant").ok();
                  }
                  "ArrowDown" | "ArrowUp" => {
                      e.prevent_default();
                      if !is_open { open_combobox(&r); }
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
                      // scroll manual dentro da lista sem mover a pagina
                      if let Ok(Some(list)) = r.query_selector("[data-rs-combobox-list]") {
                          if let Ok(list_el) = list.dyn_into::<web_sys::HtmlElement>() {
                              let item_top = items[next].offset_top();
                              let item_h = items[next].offset_height();
                              let list_scroll = list_el.scroll_top();
                              let list_h = list_el.offset_height();
                              if item_top < list_scroll {
                                  list_el.set_scroll_top(item_top);
                              } else if item_top + item_h > list_scroll + list_h {
                                  list_el.set_scroll_top(item_top + item_h - list_h);
                              }
                          }
                      }
                      update_aria_activedescendant(&r, &inp);
                  }
                  "Enter" => {
                      if !is_open { return; }
                      e.prevent_default();
                      let items = get_navigable_items(&r);
                      if let Some(focused) = items.iter().find(|el| has_state(el, "focus")) {
                          let value = focused.get_attribute("data-rs-value").unwrap_or_default();
                          let text = focused.text_content().unwrap_or_default();
                          // clear selected
                          if let Ok(all) = r.query_selector_all("[data-rs-combobox-item]") {
                              for j in 0..all.length() {
                                  if let Some(n) = all.item(j) {
                                      if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                          remove_state(&el, "selected");
                                          el.set_attribute("aria-selected", "false").ok();
                                      }
                                  }
                              }
                          }
                          add_state(focused, "selected");
                          focused.set_attribute("aria-selected", "true").ok();
                          if let Ok(Some(input_el)) = r.query_selector("[data-rs-combobox-input]") {
                              if let Ok(inp_el) = input_el.dyn_into::<web_sys::HtmlInputElement>() {
                                  inp_el.set_value(&text);
                              }
                          }
                          r.set_attribute("data-rs-value", &value).ok();
                          close_combobox(&r);
                          inp.set_attribute("aria-expanded", "false").ok();
                          inp.remove_attribute("aria-activedescendant").ok();
                          if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                              r.dispatch_event(&event).ok();
                          }
                      }
                  }
                  _ => {}
              }
          }) as Box<dyn FnMut(_)>);
          input.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).ok();
          cb.forget(); }

        // item click + hover
        if let Ok(items) = root.query_selector_all("[data-rs-combobox-item]") {
            for i in 0..items.length() {
                let Some(node) = items.item(i) else { continue };
                let Ok(item) = node.dyn_into::<web_sys::Element>() else { continue };
                if item.has_attribute("data-rs-disabled") { continue; }
                let r = root.clone();
                // click
                { let it = item.clone();
                  let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                      e.stop_propagation();
                      let value = it.get_attribute("data-rs-value").unwrap_or_default();
                      let text = it.text_content().unwrap_or_default();
                      if let Ok(all) = r.query_selector_all("[data-rs-combobox-item]") {
                          for j in 0..all.length() {
                              if let Some(n) = all.item(j) {
                                  if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                      remove_state(&el, "selected");
                                      el.set_attribute("aria-selected", "false").ok();
                                  }
                              }
                          }
                      }
                      add_state(&it, "selected");
                      it.set_attribute("aria-selected", "true").ok();
                      if let Ok(Some(inp_el)) = r.query_selector("[data-rs-combobox-input]") {
                          if let Ok(inp) = inp_el.dyn_into::<web_sys::HtmlInputElement>() {
                              inp.set_value(&text);
                          }
                      }
                      r.set_attribute("data-rs-value", &value).ok();
                      close_combobox(&r);
                      if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                          r.dispatch_event(&event).ok();
                      }
                  }) as Box<dyn FnMut(_)>);
                  item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                  cb.forget(); }
                // hover
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

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
