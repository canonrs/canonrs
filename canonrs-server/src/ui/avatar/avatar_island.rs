//! Avatar Island — Canon Rule #341
//! DOM-driven, zero state. Image load/error via web_sys + Effect.

use leptos::prelude::*;
use super::avatar_ui::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarShape, AvatarStatus};

#[component]
pub fn AvatarIsland(
    children: Children,
    #[prop(default = AvatarSize::Md)] size:      AvatarSize,
    #[prop(default = AvatarShape::Circle)] shape: AvatarShape,
    #[prop(default = AvatarStatus::Online)] status: AvatarStatus,
    #[prop(default = false)] animated:            bool,
    #[prop(default = 0i32)] badge:                i32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let img_node = match root.query_selector("[data-rs-avatar-image]").ok().flatten() {
            Some(el) => el,
            None => {
                // no image — show fallback
                if let Ok(Some(fb)) = root.query_selector("[data-rs-avatar-fallback]") {
                    let _ = fb.remove_attribute("hidden");
                    let _ = fb.set_attribute("data-rs-state", "visible");
                }
                return;
            }
        };

        let img = match img_node.dyn_into::<web_sys::HtmlImageElement>() {
            Ok(el) => el, Err(_) => return,
        };

        if img.complete() && img.natural_width() > 0 {
            let _ = root.set_attribute("data-rs-state", "loaded");
            return;
        }

        let _ = root.set_attribute("data-rs-state", "loading");

        // onload
        {
            let root_c = root.clone();
            let cb = Closure::wrap(Box::new(move || {
                let _ = root_c.set_attribute("data-rs-state", "loaded");
                if let Ok(Some(fb)) = root_c.query_selector("[data-rs-avatar-fallback]") {
                    let _ = fb.set_attribute("hidden", "");
                }
            }) as Box<dyn Fn()>);
            img.set_onload(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
        }

        // onerror
        {
            let root_c = root.clone();
            let cb = Closure::wrap(Box::new(move || {
                let _ = root_c.set_attribute("data-rs-state", "error");
                if let Ok(Some(fb)) = root_c.query_selector("[data-rs-avatar-fallback]") {
                    let _ = fb.remove_attribute("hidden");
                    let _ = fb.set_attribute("data-rs-state", "visible");
                }
            }) as Box<dyn Fn()>);
            img.set_onerror(Some(cb.as_ref().unchecked_ref()));
            cb.forget();
        }
    });

    view! {
        <div node_ref=node_ref>
            <Avatar
                size=size
                shape=shape
                status=status
                animated=animated
                badge=badge
                class=class
            >
                {children()}
            </Avatar>
        </div>
    }
}

#[component]
pub fn AvatarImageIsland(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AvatarImage src=src alt=alt class=class /> }
}

#[component]
pub fn AvatarFallbackIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AvatarFallback class=class>{children()}</AvatarFallback> }
}
