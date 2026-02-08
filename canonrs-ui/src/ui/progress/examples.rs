use leptos::prelude::*;
use super::progress_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Progress value=65.0 />
    }
}
