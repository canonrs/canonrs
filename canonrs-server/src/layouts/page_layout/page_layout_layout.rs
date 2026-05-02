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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sidebar = StoredValue::new(sidebar);
    let content = StoredValue::new(content);
    let aside   = StoredValue::new(aside);
    view! {
        <div data-rs-layout="page" data-rs-variant=variant.as_str() class=class>
            {move || sidebar.get_value().map(|s| view! { <nav data-rs-region="sidebar" aria-label="Sidebar">{s()}</nav> })}
            {move || content.get_value().map(|c| view! { <main data-rs-region="content">{c()}</main> })}
            {move || aside.get_value().map(|a| view! { <aside data-rs-region="aside" aria-label="Aside">{a()}</aside> })}
        </div>
    }
}
