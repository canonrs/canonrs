use leptos::prelude::*;
use super::Timeline;

pub fn basic_example() -> impl IntoView {
    view! {
        <Timeline
            items=leptos::children::ToChildren::to_children(|| view!{
                <div>"Event 1 — Jan 1, 2024"</div>
                <div>"Event 2 — Jan 15, 2024"</div>
                <div>"Event 3 — Feb 1, 2024"</div>
            })
        />
    }
}
