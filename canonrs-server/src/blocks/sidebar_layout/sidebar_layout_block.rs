use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid  = generate("bl");
    let nav  = StoredValue::new(nav);
    let main = StoredValue::new(main);
    view! {
        <div data-rs-sidebar-layout="" data-rs-uid=uid data-rs-side=side.as_str() class=class>
            {move || nav.get_value().map(|n| view! { <nav data-rs-region="nav" aria-label="Sidebar">{n()}</nav> })}
            {move || main.get_value().map(|m| view! { <div data-rs-region="main">{m()}</div> })}
        </div>
    }
}
