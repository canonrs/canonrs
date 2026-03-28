//! @canon-id: footer
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: left, center, right
//! @canon-label: Footer
//! @canon-description: Page footer block
//! @canon-tags: footer, bottom, links, copyright, page
//! @canon-slot-accepts: left=Nav,center=Any,right=Action
use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <footer data-rs-block="" data-rs-component="Footer" class=class>
            {left.map(|l| view! { <div data-rs-region="left">{l()}</div> })}
            {center.map(|c| view! { <div data-rs-region="center">{c()}</div> })}
            {right.map(|r| view! { <div data-rs-region="right">{r()}</div> })}
        </footer>
    }
}
