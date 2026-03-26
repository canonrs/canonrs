use leptos::prelude::*;
use super::columns_block::Columns;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Columns
            columns=leptos::children::ToChildren::to_children(|| view!{
                <div data-block-region="col-1">"Column 1"</div>
                <div data-block-region="col-2">"Column 2"</div>
            })
        />
    }
}
