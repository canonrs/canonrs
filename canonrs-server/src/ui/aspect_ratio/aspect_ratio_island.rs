use leptos::prelude::*;
use super::aspect_ratio_ui::AspectRatio;

#[component]
pub fn AspectRatioIsland(
    children: Children,
    #[prop(optional)] ratio_w: Option<f32>,
    #[prop(optional)] ratio_h: Option<f32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <AspectRatio
            ratio_w=ratio_w.unwrap_or(16.0)
            ratio_h=ratio_h.unwrap_or(9.0)
            class=class.unwrap_or_default()
        >
            {children()}
        </AspectRatio>
    }
}
