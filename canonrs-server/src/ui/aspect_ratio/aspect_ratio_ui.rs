
use leptos::prelude::*;
use canonrs_core::primitives::AspectRatioPrimitive;

#[component]
pub fn AspectRatio(
    children: Children,
    #[prop(default = 16.0f32)] ratio_w: f32,
    #[prop(default = 9.0f32)] ratio_h: f32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AspectRatioPrimitive
            ratio_w=ratio_w
            ratio_h=ratio_h
            class=class
        >
            {children()}
        </AspectRatioPrimitive>
    }
}

#[component]
pub fn AspectRatioPreview() -> impl IntoView {
    view! {
        <AspectRatio ratio_w=16.0f32 ratio_h=9.0f32>
            <div style="background:var(--theme-surface-bg-subtle);width:100%;height:100%;"></div>
        </AspectRatio>
    }
}
