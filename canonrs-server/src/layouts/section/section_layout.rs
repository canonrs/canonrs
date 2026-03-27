//! @canon-id: section
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, content, footer
//! @canon-label: Section
//! @canon-icon: ▤
//! @canon-description: Self-contained section with header, content and footer
//! @canon-tags: section, content, header, footer, container
//! @canon-slot-accepts: header=Any,content=Any,footer=Action
//! @canon-slot-descriptions: header:Section title area,content:Section content,footer:Section footer actions
//! @canon-prop: padding | Number | 2rem | visual | padding
//! @canon-prop: gap | Number | 1rem | visual | gap
//! @canon-prop: background | Color | | visual | background
use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <section data-layout="section" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-layout-region="footer">{f()}</div> })}
        </section>
    }
}
