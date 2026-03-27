//! @canon-id: data-table
//! @canon-type: block
//! @canon-category: data
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: toolbar, header, body, empty, pagination
//! @canon-label: Data Table
//! @canon-description: Sortable paginated data table block
//! @canon-tags: data-table, table, data, grid, sortable, pagination
//! @canon-slot-accepts: toolbar=Action,header=Any,body=Any,empty=Any,pagination=Nav
use leptos::prelude::*;

#[component]
pub fn DataTableBlock(
    #[prop(optional)] toolbar: Option<ChildrenFn>,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] body: Option<ChildrenFn>,
    #[prop(optional)] pagination: Option<ChildrenFn>,
    #[prop(optional)] empty: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="data-table" data-block-version="1" class=class>
            {toolbar.map(|t| view! { <div data-block-region="toolbar">{t()}</div> })}
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {body.map(|b| view! { <div data-block-region="body">{b()}</div> })}
            {empty.map(|e| view! { <div data-block-region="empty">{e()}</div> })}
            {pagination.map(|p| view! { <div data-block-region="pagination">{p()}</div> })}
        </div>
    }
}
