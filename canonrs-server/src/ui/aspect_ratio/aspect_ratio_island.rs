//! AspectRatio Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::aspect_ratio_ui::AspectRatio;

#[component]
pub fn AspectRatioIsland(
    children: Children,
    #[prop(default = 16.0f32)] ratio_w: f32,
    #[prop(default = 9.0f32)] ratio_h:  f32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AspectRatio ratio_w=ratio_w ratio_h=ratio_h class=class>
            {children()}
        </AspectRatio>
    }
}
