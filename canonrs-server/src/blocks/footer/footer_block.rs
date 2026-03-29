//! @canon-id: footer
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: top, bottom, left, center, right
//! @canon-label: Footer
//! @canon-description: Page footer block with top/bottom regions
//! @canon-tags: footer, bottom, links, copyright, page
//! @canon-slot-accepts: top=Any,bottom=Any,left=Nav,center=Any,right=Action
use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(optional)] top: Option<ChildrenFn>,
    #[prop(optional)] bottom: Option<ChildrenFn>,
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    let has_legacy = left.is_some() || center.is_some() || right.is_some();
    let has_top = top.is_some();
    view! {
        <footer data-rs-block="" data-rs-component="Footer" class=class>
            {top.map(|t| view! { <div data-rs-region="top">{t()}</div> })}
            {if !has_top && has_legacy {
                Some(view! {
                    <div data-rs-region="top">
                        {left.map(|l| view! { <div data-rs-region="left">{l()}</div> })}
                        {center.map(|c| view! { <div data-rs-region="center">{c()}</div> })}
                        {right.map(|r| view! { <div data-rs-region="right">{r()}</div> })}
                    </div>
                })
            } else {
                None
            }}
            {bottom.map(|b| view! { <div data-rs-region="bottom">{b()}</div> })}
        </footer>
    }
}
