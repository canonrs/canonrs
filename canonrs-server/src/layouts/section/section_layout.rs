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
    #[prop(optional)] header: Option<AnyView>,
    #[prop(optional)] content: Option<AnyView>,
    #[prop(optional)] footer: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <section data-rs-layout="" data-rs-component="Section" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f}</div> })}
        </section>
    }
}
