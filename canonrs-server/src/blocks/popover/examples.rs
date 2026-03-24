use leptos::prelude::*;
use super::PopoverBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <PopoverBlock open=Signal::derive(|| true)
            title=leptos::children::ToChildren::to_children(|| view!{ <span>"Popover Title"</span> })
        >
            <p>"Popover content."</p>
        </PopoverBlock>
    }
}
