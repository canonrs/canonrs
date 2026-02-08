use leptos::prelude::*;

#[component]
pub fn BannerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-banner=""
            role="region"
            attr:aria-live="polite"
            attr:data-state={if open { "open" } else { "closed" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn BannerClosePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-banner-close=""
            attr:aria-label="Close banner"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn BannerContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-banner-content=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn BannerActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-banner-actions=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
