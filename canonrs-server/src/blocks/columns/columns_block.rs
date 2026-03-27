//! @canon-id: columns
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: columns
//! @canon-label: Columns
//! @canon-description: Two equal columns
//! @canon-tags: columns,colunas,dois,split,paralelo
//! @canon-prop: gap | Number | 1rem | visual | gap
//! @canon-preset: Equal 2 | gap=1rem
//! @canon-preset: Equal 3 | gap=1rem
use leptos::prelude::*;

#[component]
pub fn Columns(
    #[prop(default = 2)] count: u8,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] columns: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="columns"
            data-block-version="1"
            style=style
            data-block-columns=count.to_string()
            class=class
        >
            {columns.map(|c| view! { <div data-block-region="columns">{c()}</div> })}
        </div>
    }
}
