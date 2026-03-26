//! # SidebarLayout Block
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
) -> impl IntoView {
    view! {
        <div
            data-block="sidebar-layout"
            data-block-version="1"
            data-block-side=side.as_str()
            class=class
        >
            {nav.map(|n| view! { <div data-block-region="nav">{n()}</div> })}
            {main.map(|m| view! { <div data-block-region="main">{m()}</div> })}
        </div>
    }
}
