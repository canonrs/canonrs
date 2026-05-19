//! @canon-level: strict
//! @canon-owner: primitives-team
//! ProgressMini Primitive - substitui div.progress-mini + div.progress-fill

use leptos::prelude::*;

#[component]
pub fn ProgressMiniPrimitive(
    #[prop(default = 0.0)] value: f64,
    #[prop(into, default = String::new())] color: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid     = crate::infra::uid::generate("prm");
    let clamped = value.clamp(0.0, 100.0);
    let fill_style = if color.is_empty() {
        format!("width:{}%", clamped)
    } else {
        format!("width:{}%; background:{}", clamped, color)
    };
    view! {
        <div
            data-rs-progress-mini=""
            data-rs-uid=uid
            data-rs-value=clamped.to_string()
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=clamped.to_string()
            class=class
        >
            <div data-rs-progress-mini-fill="" style=fill_style />
        </div>
    }
}
