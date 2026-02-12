use leptos::prelude::*;

#[component]
pub fn SidebarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <aside data-sidebar="" role="complementary" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children.map(|c| c())}
        </aside>
    }
}

#[component]
pub fn SidebarHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-header="" class={class}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-content="" class={class}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-footer="" class={class}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarMenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav data-sidebar-menu="" role="navigation" class={class}>
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn SidebarMenuItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <a data-sidebar-menu-item="" data-active={if active { "true" } else { "false" }} href={href} class={class}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn SidebarMenuGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-menu-group="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-separator="" role="separator" aria-orientation="horizontal" class={class} />
    }
}

#[component]
pub fn SidebarGroupLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-sidebar-group-label="" class={class}>
            {children.map(|c| c())}
        </div>
    }
}
