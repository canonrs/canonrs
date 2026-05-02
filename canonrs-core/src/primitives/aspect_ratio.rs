//! @canon-level: strict
//! @canon-owner: primitives-team
//! AspectRatio Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn AspectRatioPrimitive(
    children: Children,
    #[prop(default = 16.0f32)] ratio_w: f32,
    #[prop(default = 9.0f32)] ratio_h: f32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_ar = crate::infra::uid::generate("ar");
    let ratio = format!("{}/{}", ratio_w, ratio_h);
    let ratio_style = format!("aspect-ratio:{}/{}", ratio_w, ratio_h);
    view! {
        <div
            data-rs-aspect-ratio=""
            data-rs-uid=uid_ar
            data-rs-ratio=ratio
            style=ratio_style
            class=class
        >
            <div data-rs-aspect-ratio-content="">
                {children()}
            </div>
        </div>
    }
}
