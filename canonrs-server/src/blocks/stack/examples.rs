use leptos::prelude::*;
use super::Stack;

pub fn basic_example() -> impl IntoView {
    view! {
        <Stack>
            <div>"Item 1"</div>
            <div>"Item 2"</div>
            <div>"Item 3"</div>
        </Stack>
    }
}
