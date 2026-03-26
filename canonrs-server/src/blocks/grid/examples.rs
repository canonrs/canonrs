use leptos::prelude::*;
use super::grid_block::Grid;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Grid columns=3
            items=leptos::children::ToChildren::to_children(|| view!{
                <div>"Cell 1"</div>
                <div>"Cell 2"</div>
                <div>"Cell 3"</div>
            })
        />
    }
}
