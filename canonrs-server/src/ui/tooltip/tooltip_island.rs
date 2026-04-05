use leptos::prelude::*;

#[island]
pub fn TooltipIsland(
    #[prop(into)] label: String,
    #[prop(into)] content: String,
    #[prop(optional, into)] side: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let side  = side.unwrap_or_else(|| "top".to_string());
    let (is_open, set_open) = signal(false);
    let _ = set_open;

    let state = move || if is_open.get() { "open" } else { "closed" };

    #[cfg(feature = "hydrate")]
    let on_show = move |_: leptos::ev::MouseEvent| { set_open.set(true); };
    #[cfg(not(feature = "hydrate"))]
    let on_show = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_hide = move |_: leptos::ev::MouseEvent| { set_open.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_hide = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_focus = move |_: leptos::ev::FocusEvent| { set_open.set(true); };
    #[cfg(not(feature = "hydrate"))]
    let on_focus = move |_: leptos::ev::FocusEvent| {};

    #[cfg(feature = "hydrate")]
    let on_blur = move |_: leptos::ev::FocusEvent| { set_open.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_blur = move |_: leptos::ev::FocusEvent| {};

    view! {
        <span
            data-rs-tooltip=""
            data-rs-component="Tooltip"
            data-rs-state=move || state()
            class=class
        >
            <span
                data-rs-tooltip-trigger=""
                tabindex="0"
                aria-describedby="tooltip-content"
                on:mouseenter=on_show
                on:mouseleave=on_hide
                on:focus=on_focus
                on:blur=on_blur
            >
                {label}
            </span>
            <span
                data-rs-tooltip-content=""
                data-rs-side=side
                data-rs-state=move || state()
                role="tooltip"
                id="tooltip-content"
            >
                {content}
            </span>
        </span>
    }
}
