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
    #[prop(default = Signal::derive(|| String::new()))] sidebar_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] main_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] aside_zone_id: Signal<String>,
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
                    data-drop-zone=move || (!sidebar_zone_id.get().is_empty()).then_some("")
                    data-zone-id=move || (!sidebar_zone_id.get().is_empty()).then(|| sidebar_zone_id.get())>
                    {s()}
                </aside>
            })}
            <main class="layout-page-main"
                data-layout-region="main"
                data-region-hint="Drop page content"
                data-drop-zone=move || (!main_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!main_zone_id.get().is_empty()).then(|| main_zone_id.get())>
                {children()}
            </main>
            {aside.map(|a| view! {
                <aside class="layout-page-aside"
                    data-layout-region="aside"
                    data-region-hint="Drop related content"
                    data-drop-zone=move || (!aside_zone_id.get().is_empty()).then_some("")
                    data-zone-id=move || (!aside_zone_id.get().is_empty()).then(|| aside_zone_id.get())>
                    {a()}
                </aside>
            })}
        </div>
    }
}
