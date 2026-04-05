use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct CollapsibleContext {
    pub is_open:  ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
}

#[island]
pub fn CollapsibleIsland(
    children: Children,
    #[prop(optional)] open: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class        = class.unwrap_or_default();
    let initial_open = open.unwrap_or(false);
    let (is_open, set_open) = signal(initial_open);

    let initial_state = if initial_open { "open" } else { "closed" };
    let state = move || if is_open.get() { "open" } else { "closed" };

    provide_context(CollapsibleContext { is_open, set_open });

    view! {
        <div
            data-rs-collapsible=""
            data-rs-component="Collapsible"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            class=class
        >
            {children()}
        </div>
    }
}

#[island]
pub fn CollapsibleTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let ctx   = use_context::<CollapsibleContext>();
    let is_expanded = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);

    #[cfg(feature = "hydrate")]
    let on_click = move |_: leptos::ev::MouseEvent| {
        if let Some(c) = ctx { c.set_open.update(|v| *v = !*v); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| {};

    view! {
        <button
            type="button"
            data-rs-collapsible-trigger=""
            aria-expanded=move || is_expanded().to_string()
            class=class
            on:click=on_click
        >
            {children()}
        </button>
    }
}

#[island]
pub fn CollapsibleContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class    = class.unwrap_or_default();
    let ctx      = use_context::<CollapsibleContext>();
    let is_open  = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);
    let is_open2 = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);
    let is_open3 = move || ctx.map(|c| c.is_open.get()).unwrap_or(false);
    let state    = move || if is_open()  { "open" } else { "closed" };
    let hidden   = move || !is_open2();
    let aria     = move || (!is_open3()).to_string();

    view! {
        <div
            data-rs-collapsible-content=""
            data-rs-state=state
            aria-hidden=aria
            hidden=hidden
            class=class
        >
            {children()}
        </div>
    }
}
