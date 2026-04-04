use leptos::prelude::*;
use canonrs_core::primitives::{ButtonVariant as CoreVariant, ButtonSize as CoreSize};

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonVariant {
    #[default] Default,
    Primary,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

impl ButtonVariant {
    pub fn to_core(&self) -> CoreVariant {
        match self {
            Self::Default     => CoreVariant::Default,
            Self::Primary     => CoreVariant::Primary,
            Self::Secondary   => CoreVariant::Secondary,
            Self::Outline     => CoreVariant::Outline,
            Self::Ghost       => CoreVariant::Ghost,
            Self::Link        => CoreVariant::Link,
            Self::Destructive => CoreVariant::Destructive,
        }
    }
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
    pub fn to_core(&self) -> CoreSize {
        match self {
            Self::Xs   => CoreSize::Xs,
            Self::Sm   => CoreSize::Sm,
            Self::Md   => CoreSize::Md,
            Self::Lg   => CoreSize::Lg,
            Self::Xl   => CoreSize::Xl,
            Self::Icon => CoreSize::Icon,
        }
    }
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
    #[default]
    None,
    First,
    Last,
    Hover,
    Focus,
}

#[island]
pub fn ButtonIsland(
    children: Children,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] validation: Option<String>,
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
) -> impl IntoView {
    let variant  = variant.unwrap_or_default();
    let size     = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();

    let hint = state_hint.unwrap_or_default();
    let initial_hover  = hint == ButtonStateHint::Hover;
    let initial_active = false;
    let initial_focus  = hint == ButtonStateHint::Focus;
    let is_first = hint == ButtonStateHint::First;
    let is_last  = hint == ButtonStateHint::Last;
    let (is_hover,  set_hover)  = signal(initial_hover);
    let (is_active, set_active) = signal(initial_active);
    let (is_focus,  set_focus)  = signal(initial_focus);

    let state = move || {
        let mut s = vec![];
        if disabled        { s.push("disabled"); }
        if is_first        { s.push("first");    }
        if is_last         { s.push("last");     }
        if is_hover.get()  { s.push("hover");    }
        if is_active.get() { s.push("active");   }
        if is_focus.get()  { s.push("focus");    }
        s.join(" ")
    };
    let initial_state = {
        let mut s = vec![];
        if disabled        { s.push("disabled"); }
        if is_first        { s.push("first");    }
        if is_last         { s.push("last");     }
        if initial_hover   { s.push("hover");    }
        if initial_focus   { s.push("focus");    }
        s.join(" ")
    };

    // ── handlers hydrate ────────────────────────────────────
    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { if !disabled { set_hover.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { let _ = set_hover; };

    #[cfg(feature = "hydrate")]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { set_hover.set(false); set_active.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { let _ = (set_hover, set_active); };

    #[cfg(feature = "hydrate")]
    let on_pointerdown = move |_: leptos::ev::PointerEvent| { if !disabled { set_active.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_pointerdown = move |_: leptos::ev::PointerEvent| { let _ = set_active; };

    #[cfg(feature = "hydrate")]
    let on_pointerup = move |_: leptos::ev::PointerEvent| { set_active.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_pointerup = move |_: leptos::ev::PointerEvent| { let _ = set_active; };

    #[cfg(feature = "hydrate")]
    let on_pointercancel = move |_: leptos::ev::PointerEvent| { set_active.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_pointercancel = move |_: leptos::ev::PointerEvent| { let _ = set_active; };

    #[cfg(feature = "hydrate")]
    let on_focus = move |_: leptos::ev::FocusEvent| { if !disabled { set_focus.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_focus = move |_: leptos::ev::FocusEvent| { let _ = set_focus; };

    #[cfg(feature = "hydrate")]
    let on_blur = move |_: leptos::ev::FocusEvent| { set_focus.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_blur = move |_: leptos::ev::FocusEvent| { let _ = set_focus; };

    view! {
        <button
            type="button"
            data-rs-button=""
            data-rs-component="Button"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state.clone() } else { s } }
            aria-disabled=disabled.to_string()
            disabled=disabled
            aria-label=aria_label
            data-rs-validation=validation
            class=class
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
            on:pointerdown=on_pointerdown
            on:pointerup=on_pointerup
            on:pointercancel=on_pointercancel
            on:focus=on_focus
            on:blur=on_blur
        >
            {children()}
        </button>
    }
}
