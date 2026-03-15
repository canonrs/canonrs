use leptos::prelude::*;
use super::FilterBar;

pub fn basic_example() -> impl IntoView {
    view! {
        <FilterBar
            filters=leptos::children::ToChildren::to_children(|| view!{ <input type="search" placeholder="Search..." /> })
            actions=leptos::children::ToChildren::to_children(|| view!{ <button>"Apply"</button> })
        />
    }
}
