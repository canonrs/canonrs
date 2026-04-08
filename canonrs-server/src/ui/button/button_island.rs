//! Button Island — Canon Rule #341 + #342
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonVariant {
    #[default] Default,
    Primary, Secondary, Outline, Ghost, Link, Destructive,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Primary     => "primary",
            Self::Secondary   => "secondary",
            Self::Outline     => "outline",
            Self::Ghost       => "ghost",
            Self::Link        => "link",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonSize {
    Xs, Sm, #[default] Md, Lg, Xl, Icon,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs   => "xs",
            Self::Sm   => "sm",
            Self::Md   => "md",
            Self::Lg   => "lg",
            Self::Xl   => "xl",
            Self::Icon => "icon",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonStateHint {
    #[default] None,
    First, Last, Hover, Focus,
}

#[island]
pub fn ButtonIsland(
    children: Children,
    #[prop(optional)] variant:    Option<ButtonVariant>,
    #[prop(optional)] size:       Option<ButtonSize>,
    #[prop(optional)] disabled:   Option<bool>,
    #[prop(optional, into)] class:       Option<String>,
    #[prop(optional, into)] aria_label:  Option<String>,
    #[prop(optional, into)] validation:  Option<String>,
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
) -> impl IntoView {
    let variant  = variant.unwrap_or_default();
    let size     = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();
    let hint     = state_hint.unwrap_or_default();

    let initial_state = {
        let mut s = vec![];
        if disabled                          { s.push("disabled"); }
        if hint == ButtonStateHint::First    { s.push("first");    }
        if hint == ButtonStateHint::Last     { s.push("last");     }
        if hint == ButtonStateHint::Hover    { s.push("hover");    }
        if hint == ButtonStateHint::Focus    { s.push("focus");    }
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
            let next = cur.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
            let _ = el.set_attribute("data-rs-state", &next);
        };

        // mouseenter → hover
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
              if r.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
              add(&r, "hover");
          }));
          let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // mouseleave → remove hover + active
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
              remove(&r, "hover"); remove(&r, "active");
          }));
          let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // pointerdown → active
        { let r = root.clone();
          let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::PointerEvent| {
              if r.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
              add(&r, "active");
          }));
          let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
          cb.forget(); }

        // pointerup + pointercancel → remove active
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
    });

    view! {
        <button
            type="button"
            data-rs-button=""
            data-rs-component="Button"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-state=initial_state
            aria-disabled=disabled.to_string()
            disabled=disabled
            aria-label=aria_label
            data-rs-validation=validation
            class=class
            node_ref=node_ref
        >
            {children()}
        </button>
    }
}
