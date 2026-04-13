use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

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
    let uid     = generate("ly");
    let sidebar = StoredValue::new(sidebar);
    let content = StoredValue::new(content);
    let aside   = StoredValue::new(aside);
    view! {
        <div data-rs-layout-page="" data-rs-uid=uid data-rs-variant=variant.as_str() class=class>
            {move || sidebar.get_value().map(|s| view! { <div data-rs-region="sidebar">{s()}</div> })}
            {move || content.get_value().map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {move || aside.get_value().map(|a| view! { <div data-rs-region="aside">{a()}</div> })}
        </div>
    }
}
