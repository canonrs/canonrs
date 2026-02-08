use leptos::prelude::*;
use super::loading_overlay_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <LoadingOverlay>"Loading..."</LoadingOverlay>
    }
}
