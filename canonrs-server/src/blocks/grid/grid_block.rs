//! @canon-id: grid
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: items
//! @canon-prop: grid-columns | Number | 3 | structural | 
//! @canon-prop: grid-template-columns | Select(repeat(1,1fr):1,repeat(2,1fr):2,repeat(3,1fr):3,repeat(4,1fr):4,repeat(5,1fr):5,repeat(6,1fr):6) | repeat(3,1fr) | visual | grid-template-columns
//! @canon-prop: gap | Number | 1rem | visual | gap
//! @canon-prop: row-gap | Number | | visual | row-gap
//! @canon-preset: 2 Columns | grid-columns=2,grid-template-columns=repeat(2,1fr),gap=1rem
//! @canon-preset: 3 Columns | grid-columns=3,grid-template-columns=repeat(3,1fr),gap=1rem
//! @canon-preset: 4 Columns | grid-columns=4,grid-template-columns=repeat(4,1fr),gap=1rem
//! @canon-preset: Sidebar | grid-template-columns=240px 1fr,gap=1.5rem
use leptos::prelude::*;

#[component]
pub fn Grid(
    #[prop(default = 3u8)] columns: u8,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="grid"
            data-block-version="1"
            style=style
            data-block-columns=columns.to_string()
            class=class
        >
            {items.map(|i| view! { <div data-block-region="items">{i()}</div> })}
        </div>
    }
}
