use leptos::prelude::*;
use canonrs_core::primitives::AspectRatioPrimitive;

#[component]
pub fn AspectRatio(
    children: Children,
    #[prop(default = 16.0)] width: f32,
    #[prop(default = 9.0)] height: f32,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let ratio = format!("{}/{}", width as i32, height as i32);
    
    view! {
        <AspectRatioPrimitive
            ratio={ratio}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </AspectRatioPrimitive>
    }
}

#[component]
pub fn AspectRatioPreview() -> impl IntoView {
    view! { <AspectRatio width=16.0 height=9.0><div style="background:var(--theme-surface-bg-subtle);width:100%;height:100%;"></div></AspectRatio> }
}
