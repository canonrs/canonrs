//! @canon-id: fullscreen
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, content
//! @canon-tags: fullscreen,full,viewport,tela cheia,imersivo
//! @canon-label: Fullscreen
//! @canon-icon: ⬜
//! @canon-description: Optional header with full canvas content area
//! @canon-slot-descriptions: header:Optional top bar,content:Full canvas area
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-layout="fullscreen" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
        </div>
    }
}
