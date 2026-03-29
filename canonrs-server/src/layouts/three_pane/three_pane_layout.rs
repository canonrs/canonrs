//! @canon-id: three-pane
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: left, center, right
//! @canon-label: Three Pane
//! @canon-icon: ⊞
//! @canon-description: Global nav left, content center, local nav right
//! @canon-tags: three-pane, three-columns, docs, toc, enterprise
//! @canon-slot-accepts: left=Nav,center=Content,right=Toc
//! @canon-slot-descriptions: left:Global navigation,center:Main content,right:Local TOC
use leptos::prelude::*;

#[component]
pub fn ThreePaneLayout(
    #[prop(optional)] left: Option<AnyView>,
    #[prop(optional)] center: Option<AnyView>,
    #[prop(optional)] right: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-layout=""
            data-rs-component="ThreePane"
            class=class
        >
            {left.map(|l| view! { <div data-rs-region="left">{l}</div> })}
            {center.map(|c| view! { <div data-rs-region="center">{c}</div> })}
            {right.map(|r| view! { <div data-rs-region="right">{r}</div> })}
        </div>
    }
}
