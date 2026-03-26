use leptos::prelude::*;
use super::toolbar_block::ToolbarBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <ToolbarBlock
            left=leptos::children::ToChildren::to_children(|| view!{ <button>"Back"</button> })
            right=leptos::children::ToChildren::to_children(|| view!{ <button>"Save"</button> })
            center=leptos::children::ToChildren::to_children(|| view!{ <span>"Toolbar"</span> })
        />
    }
}
