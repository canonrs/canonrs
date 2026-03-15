use leptos::prelude::*;
use super::Container;

pub fn basic_example() -> impl IntoView {
    view! {
        <Container>
            <p>"Centered container content."</p>
        </Container>
    }
}
