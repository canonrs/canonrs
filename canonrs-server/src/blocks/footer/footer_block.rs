//! @canon-id: footer
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: top, bottom, left, center, right
//! @canon-label: Footer
//! @canon-description: Page footer block with top/bottom regions. top accepts left/center/right sub-regions for legacy compat.
//! @canon-tags: footer, bottom, links, copyright, page
//! @canon-slot-accepts: top=Any,bottom=Any,left=Nav,center=Any,right=Action
use leptos::prelude::*;

#[component]
pub fn Footer(
    // New regions
    #[prop(optional)] top: Option<ChildrenFn>,
    #[prop(optional)] bottom: Option<ChildrenFn>,
    // Legacy compat
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <footer data-rs-block="" data-rs-component="Footer" class=class>
            {move || {
                if top.is_some() {
                    top.as_ref().map(|t| view! { <div data-rs-region="top">{t()}</div> })
                } else if left.is_some() || center.is_some() || right.is_some() {
                    Some(view! {
                        <div data-rs-region="top">
                            {left.as_ref().map(|l| view! { <div data-rs-region="left">{l()}</div> })}
                            {center.as_ref().map(|c| view! { <div data-rs-region="center">{c()}</div> })}
                            {right.as_ref().map(|r| view! { <div data-rs-region="right">{r()}</div> })}
                        </div>
                    })
                } else {
                    None
                }
            }}
            {bottom.map(|b| view! { <div data-rs-region="bottom">{b()}</div> })}
        </footer>
    }
}
