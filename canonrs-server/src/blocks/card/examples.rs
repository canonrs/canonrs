use leptos::prelude::*;
use super::card_block::Card;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Card
            header=leptos::children::ToChildren::to_children(|| view!{ <h3>"Card Title"</h3> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Card content goes here."</p> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <button>"Action"</button> })
        />
    }
}
