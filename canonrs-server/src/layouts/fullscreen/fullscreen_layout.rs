//! @canon-id: fullscreen
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, content
//! @canon-label: Fullscreen
//! @canon-icon: ⬜
//! @canon-description: Optional header with full canvas content area
//! @canon-tags: fullscreen, full, viewport, immersive, canvas
//! @canon-slot-accepts: header=Nav,content=Any
//! @canon-slot-descriptions: header:Optional top bar,content:Full canvas area
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<AnyView>,
    #[prop(optional)] content: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-layout="" data-rs-component="Fullscreen" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c}</div> })}
        </div>
    }
}
