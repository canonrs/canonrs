use leptos::prelude::*;
use super::ToolbarBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <ToolbarBlock
            left=leptos::children::ToChildren::to_children(|| view!{ <button>"Back"</button> })
            right=leptos::children::ToChildren::to_children(|| view!{ <button>"Save"</button> })
        >
            <span>"Toolbar"</span>
        </ToolbarBlock>
    }
}
