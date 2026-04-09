//! IconButton Island — Canon Rule #341
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

use canonrs_core::primitives::{IconButtonVariant, IconButtonSize};

#[island]
pub fn IconButtonIsland(
    children: Children,
    #[prop(optional, into)] aria_label:  Option<String>,
    #[prop(optional)] variant:           Option<IconButtonVariant>,
    #[prop(optional)] size:              Option<IconButtonSize>,
    #[prop(optional)] disabled:          Option<bool>,
    #[prop(optional)] pressed:           Option<bool>,
    #[prop(optional)] loading:           Option<bool>,
    #[prop(optional, into)] class:       Option<String>,
) -> impl IntoView {
    let aria_label = aria_label.unwrap_or_default();
    let variant    = variant.unwrap_or_default();
    let size       = size.unwrap_or_default();
    let disabled   = disabled.unwrap_or(false);
    let pressed    = pressed.unwrap_or(false);
    let loading    = loading.unwrap_or(false);
    let class      = class.unwrap_or_default();

    let initial_state = {
        let mut s = vec![];
        if disabled { s.push("disabled"); }
        if pressed  { s.push("pressed");  }
        if loading  { s.push("loading");  }
        s.join(" ")
    };

    let node_ref = NodeRef::<leptos::html::Button>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let add = |el: &web_sys::Element, token: &str| {
            let cur = el.get_attribute("data-rs-state").unwrap_or_default();
            if cur.split_whitespace().any(|t| t == token) { return; }
            let _ = el.set_attribute("data-rs-state", &format!("{} {}", cur, token).trim().to_string());
        };
        let remove = |el: &web_sys::Element, token: &str| {
            let cur = el.get_attribute("data-rs-state").unwrap_or_default();
            let _ = el.set_attribute("data-rs-state", &cur.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" "));
        };

        // mouseenter
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
              if r.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
              add(&r, "hover");
          }));
          let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // mouseleave
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
              remove(&r, "hover"); remove(&r, "active");
          }));
          let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // pointerdown
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::PointerEvent| {
              if r.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
              add(&r, "active");
          }));
          let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // pointerup + pointercancel
        for ev in ["pointerup", "pointercancel"] {
            let r = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::PointerEvent| {
                remove(&r, "active");
            }));
            let _ = root.add_event_listener_with_callback(ev, cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // focus
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
              if r.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
              add(&r, "focus");
          }));
          let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // blur
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
              remove(&r, "focus");
          }));
          let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // click → toggle pressed
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
              let state = r.get_attribute("data-rs-state").unwrap_or_default();
              if state.contains("disabled") { return; }
              if state.contains("pressed") {
                  remove(&r, "pressed");
                  let _ = r.set_attribute("aria-pressed", "false");
              } else {
                  add(&r, "pressed");
                  let _ = r.set_attribute("aria-pressed", "true");
              }
          }));
          let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
          cb.forget(); }
    });

    view! {
        <button
            type="button"
            data-rs-icon-button=""
            data-rs-component="IconButton"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-state=initial_state
            aria-disabled=disabled.to_string()
            aria-pressed=pressed.to_string()
            aria-label=aria_label
            disabled=disabled
            class=class
            node_ref=node_ref
        >
            <span data-rs-icon-button-inner="">{children()}</span>
        </button>
    }
}
