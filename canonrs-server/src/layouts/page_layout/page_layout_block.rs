//! # PageLayout — Regions: sidebar, content, aside
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
