use leptos::prelude::*;
use super::DetailPanel;

pub fn basic_example() -> impl IntoView {
    view! {
        <DetailPanel
            aside=leptos::children::ToChildren::to_children(|| view!{ <div>"Sidebar content"</div> })
            content=leptos::children::ToChildren::to_children(|| view!{ <div>"Main content"</div> })
        />
    }
}
