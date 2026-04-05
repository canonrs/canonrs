use leptos::prelude::*;

#[island]
pub fn HoverCardIsland(
    #[prop(into)] trigger: String,
    #[prop(into)] content: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
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
            data-rs-hover-card=""
            data-rs-component="HoverCard"
            data-rs-state=move || state()
            class=class
        >
            <span
                data-rs-hover-card-trigger=""
                tabindex="0"
                on:mouseenter=on_show
                on:mouseleave=on_hide
                on:focus=on_focus
                on:blur=on_blur
            >
                {trigger}
            </span>
            <div
                data-rs-hover-card-content=""
                data-rs-state=move || state()
                role="region"
            >
                {content}
            </div>
        </span>
    }
}
