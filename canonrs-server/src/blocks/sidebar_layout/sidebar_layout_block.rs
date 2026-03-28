//! @canon-id: sidebar-layout
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: nav, main
//! @canon-label: Sidebar Layout
//! @canon-description: Block-level sidebar and main content
//! @canon-tags: sidebar-layout, sidebar, nav, menu, lateral
//! @canon-slot-accepts: nav=Nav,main=Any
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SidebarSide { #[default] Left, Right }
impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Left => "left", Self::Right => "right" }
    }
}

#[component]
pub fn SidebarLayout(
    #[prop(default = SidebarSide::Left)] side: SidebarSide,
    #[prop(optional)] nav: Option<ChildrenFn>,
    #[prop(optional)] main: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="SidebarLayout"
            data-rs-side=side.as_str()
            style=style
            class=class
        >
            {nav.map(|n| view! { <div data-rs-region="nav">{n()}</div> })}
            {main.map(|m| view! { <div data-rs-region="main">{m()}</div> })}
        </div>
    }
}
