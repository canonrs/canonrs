use leptos::prelude::*;
use super::Toggle;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Toggle>"Bold"</Toggle>
    }
}
