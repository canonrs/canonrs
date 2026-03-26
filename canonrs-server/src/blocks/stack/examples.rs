use leptos::prelude::*;
use super::stack_block::Stack;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Stack
            items=leptos::children::ToChildren::to_children(|| view!{
                <div>"Item 1"</div>
                <div>"Item 2"</div>
                <div>"Item 3"</div>
            })
        />
    }
}
