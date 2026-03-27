//! @canon-id: detail-panel
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: aside, content
//! @canon-label: Detail Panel
//! @canon-description: Master-detail panel layout
//! @canon-tags: detail,detalhe,master,painel,inspector
use leptos::prelude::*;

#[component]
pub fn DetailPanel(
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="detail-panel" data-block-version="1" class=class>
            {aside.map(|a| view! { <div data-block-region="aside">{a()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
        </div>
    }
}
