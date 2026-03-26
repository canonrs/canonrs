use leptos::prelude::*;
use super::list_block::List;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <List
            items=leptos::children::ToChildren::to_children(|| view!{
                <div>"Item 1"</div>
                <div>"Item 2"</div>
                <div>"Item 3"</div>
            })
        />
    }
}
