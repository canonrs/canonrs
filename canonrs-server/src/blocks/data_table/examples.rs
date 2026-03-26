use leptos::prelude::*;
use super::data_table_block::DataTableBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <DataTableBlock
            toolbar=leptos::children::ToChildren::to_children(|| view!{ <div>"Search + Filters"</div> })
            header=leptos::children::ToChildren::to_children(|| view!{ <div>"Name | Status | Date"</div> })
            body=leptos::children::ToChildren::to_children(|| view!{ <div>"Row data here"</div> })
            pagination=leptos::children::ToChildren::to_children(|| view!{ <div>"Page 1 of 10"</div> })
        />
    }
}
