use leptos::prelude::*;
use super::spinner_ui::Spinner;
use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn SpinnerIsland(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Spinner size=size paused=paused aria_label=aria_label class=class />
    }
}
