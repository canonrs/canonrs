use leptos::prelude::*;
use super::separator_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <p>"Above separator"</p>
            <Separator />
            <p>"Below separator"</p>
        </div>
    }
}
