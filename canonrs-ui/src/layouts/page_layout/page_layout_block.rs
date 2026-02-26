//! # PageLayout — each variant is a distinct layout
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PageLayoutVariant { Single, WithSidebar, WithAside, SidebarAndAside }

impl PageLayoutVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single          => "page-single",
            Self::WithSidebar     => "page-with-sidebar",
            Self::WithAside       => "page-with-aside",
            Self::SidebarAndAside => "page-sidebar-and-aside",
        }
    }
}

#[component]
pub fn PageLayout(
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(default = PageLayoutVariant::Single)] variant: PageLayoutVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    children: Children,
) -> impl IntoView {
    let layout_id = variant.as_str();
    view! {
        <div class={format!("layout-page {}", class)} id={id}
            data-layout={layout_id} data-layout-version="1">
            {sidebar.map(|s| view! {
                <aside class="layout-page-sidebar"
                    data-layout-region="sidebar"
                    data-region-hint="Drop navigation"
                    data-region-meta="Nav · max 1">
                    {s()}
                </aside>
            })}
            <main class="layout-page-main"
                data-layout-region="main"
                data-region-hint="Drop page content"
                data-region-meta="Content · ∞">
                {children()}
            </main>
            {aside.map(|a| view! {
                <aside class="layout-page-aside"
                    data-layout-region="aside"
                    data-region-hint="Drop related content"
                    data-region-meta="Content · max 1">
                    {a()}
                </aside>
            })}
        </div>
    }
}
