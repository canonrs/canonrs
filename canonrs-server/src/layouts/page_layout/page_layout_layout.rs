//! @canon-id: page-layout
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: sidebar, content, aside
//! @canon-label: Page
//! @canon-icon: ▭
//! @canon-description: Page layout with optional sidebar and aside
//! @canon-tags: page, single, column, article, content, layout
//! @canon-slot-accepts: sidebar=Nav,content=Any,aside=Any
//! @canon-slot-descriptions: sidebar:Navigation sidebar,content:Primary content,aside:Contextual panel
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PageLayoutVariant { #[default] Single, WithSidebar, WithAside, SidebarAndAside }
impl PageLayoutVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single          => "single",
            Self::WithSidebar     => "with-sidebar",
            Self::WithAside       => "with-aside",
            Self::SidebarAndAside => "sidebar-and-aside",
        }
    }
}

#[component]
pub fn PageLayout(
    #[prop(default = PageLayoutVariant::Single)] variant: PageLayoutVariant,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            data-layout="page"
            data-layout-variant=variant.as_str()
            data-layout-version="1"
            class=class
        >
            {sidebar.map(|s| view! { <div data-layout-region="sidebar">{s()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
            {aside.map(|a| view! { <div data-layout-region="aside">{a()}</div> })}
        </div>
    }
}
