use leptos::prelude::*;
use super::DialogBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <DialogBlock open=Signal::derive(|| true)
            title=leptos::children::ToChildren::to_children(|| view!{ <span>"Dialog Title"</span> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <button>"Confirm"</button> })
        >
            <p>"Dialog content goes here."</p>
        </DialogBlock>
    }
}
