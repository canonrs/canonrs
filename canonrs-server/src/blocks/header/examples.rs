use leptos::prelude::*;
use super::Header;

pub fn basic_example() -> impl IntoView {
    view! {
        <Header
            logo=leptos::children::ToChildren::to_children(|| view!{ <span>"CanonRS"</span> })
            actions=leptos::children::ToChildren::to_children(|| view!{ <button>"Login"</button> })
        />
    }
}
