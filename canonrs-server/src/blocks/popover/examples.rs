use leptos::prelude::*;
use super::popover_block::PopoverBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <PopoverBlock
            trigger=leptos::children::ToChildren::to_children(|| view!{ <button>"Open"</button> })
            header=leptos::children::ToChildren::to_children(|| view!{ <h4>"Popover Title"</h4> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Popover content"</p> })
        />
    }
}
