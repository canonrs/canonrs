//! @canon-id: dialog
//! @canon-type: block
//! @canon-category: overlay
//! @canon-variant: overlay
//! @canon-container: true
//! @canon-regions: header, content, footer
//! @canon-label: Dialog
//! @canon-description: Modal dialog overlay block
//! @canon-tags: dialog, modal, popup, overlay, window, alert
//! @canon-slot-accepts: header=Any,content=Any,footer=Action
use leptos::prelude::*;

#[component]
pub fn DialogBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div data-rs-block="" data-rs-component="Dialog" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
