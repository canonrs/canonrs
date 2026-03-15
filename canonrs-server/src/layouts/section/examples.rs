use leptos::prelude::*;
use super::Section;

pub fn basic_example() -> impl IntoView {
    view! {
        <Section
            header=leptos::children::ToChildren::to_children(|| view!{ <h2>"Section Title"</h2> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <div>"Section footer"</div> })
        >
            <p>"Section content"</p>
        </Section>
    }
}
