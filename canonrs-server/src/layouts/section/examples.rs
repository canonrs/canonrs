use leptos::prelude::*;
use super::Section;

pub fn basic_example() -> impl IntoView {
    view! {
        <Section
            header=leptos::children::ToChildren::to_children(|| view!{ <h2>"Section Title"</h2> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Section content"</p> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <div>"Section footer"</div> })
        />
    }
}
