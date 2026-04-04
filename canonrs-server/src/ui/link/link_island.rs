use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum LinkVariant {
    #[default]
    Default,
    Muted,
    Underline,
}

impl LinkVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default   => "default",
            Self::Muted     => "muted",
            Self::Underline => "underline",
        }
    }
}

#[island]
pub fn LinkIsland(
    children: Children,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] variant: Option<LinkVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] external: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let href     = href.unwrap_or_default();
    let variant  = variant.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let external = external.unwrap_or(false);
    let class    = class.unwrap_or_default();

    let target = if external { "_blank" } else { "" };
    let rel    = if external { "noopener noreferrer" } else { "" };

    let (is_hover,  set_hover)  = signal(false);
    let (is_active, set_active) = signal(false);

    let initial_state = {
        let mut s = vec![];
        if disabled { s.push("disabled"); }
        s.join(" ")
    };

    let state = move || {
        let mut s = vec![];
        if disabled        { s.push("disabled"); }
        if is_hover.get()  { s.push("hover");    }
        if is_active.get() { s.push("active");   }
        s.join(" ")
    };

    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { if !disabled { set_hover.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { let _ = set_hover; };

    #[cfg(feature = "hydrate")]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { set_hover.set(false); set_active.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { let _ = (set_hover, set_active); };

    #[cfg(feature = "hydrate")]
    let on_mousedown = move |_: leptos::ev::MouseEvent| { if !disabled { set_active.set(true); } };
    #[cfg(not(feature = "hydrate"))]
    let on_mousedown = move |_: leptos::ev::MouseEvent| { let _ = set_active; };

    #[cfg(feature = "hydrate")]
    let on_mouseup = move |_: leptos::ev::MouseEvent| { set_active.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseup = move |_: leptos::ev::MouseEvent| { let _ = set_active; };

    view! {
        <a
            data-rs-link=""
            data-rs-component="Link"
            data-rs-variant=variant.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state.clone() } else { s } }
            href=if disabled { "#".to_string() } else { href }
            target=target
            rel=rel
            aria-disabled=disabled.to_string()
            class=class
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
            on:mousedown=on_mousedown
            on:mouseup=on_mouseup
        >
            {children()}
        </a>
    }
}
