use leptos::prelude::*;
use canonrs_core::primitives::AspectRatioPrimitive;

#[component]
pub fn AspectRatio(
    children: Children,
    #[prop(default = 16.0f32)] width: f32,
    #[prop(default = 9.0f32)] height: f32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AspectRatioPrimitive
            ratio_w=width
            ratio_h=height
            class=class
        >
            {children()}
        </AspectRatioPrimitive>
    }
}

#[component]
pub fn AspectRatioPreview() -> impl IntoView {
    view! {
        <AspectRatio width=16.0f32 height=9.0f32>
            <div style="background:var(--theme-surface-bg-subtle);width:100%;height:100%;"></div>
        </AspectRatio>
    }
}
