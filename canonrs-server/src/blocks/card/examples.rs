use leptos::prelude::*;
use super::Card;

pub fn basic_example() -> impl IntoView {
    view! {
        <Card>
            <p>"Card content goes here."</p>
        </Card>
    }
}

pub fn with_header_footer_example() -> impl IntoView {
    view! {
        <Card
            header=leptos::children::ToChildren::to_children(|| view!{ <h3>"Card Title"</h3> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <div>"Footer actions"</div> })
        >
            <p>"Card body content."</p>
        </Card>
    }
}
