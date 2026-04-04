use leptos::prelude::*;
use canonrs_core::primitives::IconButtonVariant as CoreVariant;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum IconButtonVariant {
    #[default] Default,
    Ghost,
    Outline,
    Solid,
    Subtle,
    Destructive,
}

impl IconButtonVariant {
    pub fn to_core(&self) -> CoreVariant {
        match self {
            Self::Default     => CoreVariant::Default,
            Self::Ghost       => CoreVariant::Ghost,
            Self::Outline     => CoreVariant::Outline,
            Self::Solid       => CoreVariant::Solid,
            Self::Subtle      => CoreVariant::Subtle,
            Self::Destructive => CoreVariant::Destructive,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Ghost       => "ghost",
            Self::Outline     => "outline",
            Self::Solid       => "solid",
            Self::Subtle      => "subtle",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum IconButtonSize {
    Xs, Sm, #[default] Md, Lg, Xl,
}

impl IconButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

#[island]
pub fn IconButtonIsland(
    children: Children,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] variant: Option<IconButtonVariant>,
    #[prop(optional)] size: Option<IconButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] pressed: Option<bool>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant  = variant.unwrap_or_default();
    let size     = size.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();

    let (is_hover,   set_hover)   = signal(false);
    let (is_active,  set_active)  = signal(false);
    let (is_focus,   set_focus)   = signal(false);
    let (is_pressed, set_pressed) = signal(pressed.unwrap_or(false));
    let loading = loading.unwrap_or(false);

    let state = move || {
        let mut s = vec![];
        if disabled          { s.push("disabled"); }
        if is_hover.get()    { s.push("hover");    }
        if is_active.get()   { s.push("active");   }
        if is_focus.get()    { s.push("focus");    }
        if is_pressed.get()  { s.push("pressed");  }
        if loading           { s.push("loading");  }
        s.join(" ")
    };

    let initial_state = {
        let mut s = vec![];
        if disabled              { s.push("disabled"); }
        if pressed.unwrap_or(false) { s.push("pressed"); }
        if loading             { s.push("loading"); }
        s.join(" ")
    };

    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { if !disabled && !loading { set_hover.set(true); } };
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

    #[cfg(feature = "hydrate")]
    let on_click = move |_: leptos::ev::MouseEvent| {
        if !disabled { set_pressed.update(|p| *p = !*p); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| { let _ = set_pressed; };

    view! {
        <button
            type="button"
            data-rs-icon-button=""
            data-rs-component="IconButton"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state.clone() } else { s } }
            aria-disabled=disabled.to_string()
            aria-pressed=is_pressed.get().to_string()
            aria-label=aria_label
            disabled=disabled
            class=class
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
            on:pointerdown=on_pointerdown
            on:pointerup=on_pointerup
            on:pointercancel=on_pointercancel
            on:focus=on_focus
            on:blur=on_blur
            on:click=on_click
        >
            <span data-rs-icon-button-inner="">{children()}</span>
        </button>
    }
}
