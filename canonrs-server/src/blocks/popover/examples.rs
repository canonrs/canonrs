use leptos::prelude::*;
use leptos::reactive::wrappers::read::MaybeSignal;
use super::PopoverBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <PopoverBlock open=MaybeSignal::Static(true)
            title=leptos::children::ToChildren::to_children(|| view!{ <span>"Popover Title"</span> })
        >
            <p>"Popover content."</p>
        </PopoverBlock>
    }
}
