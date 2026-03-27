//! @canon-id: timeline
//! @canon-type: block
//! @canon-category: data
//! @canon-variant: feature
//! @canon-container: true
//! @canon-regions: header, items, footer
//! @canon-label: Timeline
//! @canon-description: Chronological timeline block
//! @canon-tags: timeline, chronology, history, events, log
//! @canon-slot-accepts: header=Any,items=Any,footer=Action
use leptos::prelude::*;

#[component]
pub fn Timeline(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div data-block="timeline" data-block-version="1" class=class>
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {items.map(|i| view! { <div data-block-region="items">{i()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
