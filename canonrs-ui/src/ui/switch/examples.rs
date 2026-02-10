use leptos::prelude::*;
use super::Switch;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Switch checked=false />
    }
}
