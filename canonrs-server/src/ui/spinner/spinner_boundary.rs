use leptos::prelude::*;
use super::spinner_ui::Spinner as SpinnerUi;
use canonrs_core::primitives::SpinnerSize;

#[component]
pub fn Spinner(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SpinnerUi size=size paused=paused aria_label=aria_label class=class />
};
}
