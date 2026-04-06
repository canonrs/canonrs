use leptos::prelude::*;
use super::progress_ui::Progress;

#[island]
pub fn ProgressIsland(
    #[prop(optional)] value: Option<f64>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let val = value.unwrap_or(0.0).clamp(0.0, 100.0);
    let cls = class.unwrap_or_default();

    view! {
        <Progress value=val class=cls />
    }
}
