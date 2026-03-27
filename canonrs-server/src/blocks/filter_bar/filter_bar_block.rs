//! @canon-id: filter-bar
//! @canon-type: block
//! @canon-category: data
//! @canon-variant: feature
//! @canon-container: true
//! @canon-regions: filters, actions
//! @canon-label: Filter Bar
//! @canon-description: Filters and actions bar
//! @canon-tags: filter-bar, filter, search, bar, refinement
//! @canon-slot-accepts: filters=Form,actions=Action
use leptos::prelude::*;

#[component]
pub fn FilterBar(
    #[prop(optional)] filters: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="filter-bar" data-block-version="1" class=class>
            {filters.map(|f| view! { <div data-block-region="filters">{f()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </div>
    }
}
