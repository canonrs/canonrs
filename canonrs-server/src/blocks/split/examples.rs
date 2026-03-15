use leptos::prelude::*;
use super::Split;

pub fn basic_example() -> impl IntoView {
    view! {
        <Split
            aside=leptos::children::ToChildren::to_children(|| view!{ <div>"Aside panel"</div> })
            main=leptos::children::ToChildren::to_children(|| view!{ <div>"Main panel"</div> })
        />
    }
}
