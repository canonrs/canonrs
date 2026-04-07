use leptos::prelude::*;
use super::avatar_ui::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarShape, AvatarStatus};

#[island]
pub fn AvatarInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        let f = Closure::wrap(Box::new(move || {
            crate::interactions::avatar::init_all();
        }) as Box<dyn Fn()>);
        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();
        f.forget();
    }
    view! { <></> }
}

#[component]
pub fn AvatarIsland(
    children: Children,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] shape: Option<String>,
    #[prop(optional, into)] status: Option<String>,
    #[prop(optional)] animated: Option<bool>,
    #[prop(optional)] badge: Option<i32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size_val = match size.as_deref() {
        Some("xs") => AvatarSize::Xs,
        Some("sm") => AvatarSize::Sm,
        Some("lg") => AvatarSize::Lg,
        Some("xl") => AvatarSize::Xl,
        _          => AvatarSize::Md,
    };
    let shape_val = match shape.as_deref() {
        Some("square")  => AvatarShape::Square,
        Some("rounded") => AvatarShape::Rounded,
        _               => AvatarShape::Circle,
    };
    let status_val: Option<AvatarStatus> = match status.as_deref() {
        Some("online")  => Some(AvatarStatus::Online),
        Some("offline") => Some(AvatarStatus::Offline),
        Some("busy")    => Some(AvatarStatus::Busy),
        Some("away")    => Some(AvatarStatus::Away),
        _               => None,
    };
    view! {
        <AvatarInit />
        <Avatar
            size=size_val
            shape=shape_val
            status=status_val.unwrap_or(AvatarStatus::Online)
            animated=animated.unwrap_or(false)
            badge=badge.unwrap_or(0)
            class=class.unwrap_or_default()
        >
            {children()}
        </Avatar>
    }
}

#[component]
pub fn AvatarImageIsland(
    src: String,
    alt: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <AvatarImage src=src alt=alt class=class.unwrap_or_default() /> }
}

#[component]
pub fn AvatarFallbackIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <AvatarFallback class=class.unwrap_or_default()>{children()}</AvatarFallback> }
}
