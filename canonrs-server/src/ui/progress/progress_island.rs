//! @canon-level: strict
//! Progress Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::progress_ui::Progress;

#[component]
pub fn ProgressIsland(
    #[prop(default = 0.0)] value: f64,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Progress value=value.clamp(0.0, 100.0) class=class /> }
}
