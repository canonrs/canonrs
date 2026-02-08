use leptos::prelude::*;
use crate::shared::{Orientation, DrawerVariant};
use crate::ui::drawer::{Drawer, DrawerTrigger, DrawerContent};
use crate::primitives::button::ButtonPrimitive;
use crate::primitives::label::LabelPrimitive;
use crate::primitives::separator::SeparatorPrimitive;

#[component]
pub fn Sidebar(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    collapsed: Signal<bool>,
    #[prop(default = DrawerVariant::Persistent)] variant: DrawerVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <Drawer
            open={open}
            collapsed={collapsed}
            variant={variant}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </Drawer>
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerTrigger class={class} id={id}>
            {children.map(|c| c())}
        </DrawerTrigger>
    }
}

#[component]
pub fn SidebarContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerContent class={class} id={id}>
            {children.map(|c| c())}
        </DrawerContent>
    }
}

#[component]
pub fn SidebarGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] labelledby: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-sidebar-group=""
            role="group"
            attr:aria-labelledby={(!labelledby.is_empty()).then(|| labelledby)}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <LabelPrimitive
            attr:data-sidebar-group-label=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </LabelPrimitive>
    }
}

#[component]
pub fn SidebarSeparator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SeparatorPrimitive
            orientation={orientation.as_str().to_string()}
            decorative={true}
            class={class}
            attr:data-sidebar-separator=""
        />
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            attr:data-sidebar-menu=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-sidebar-menu-item=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarMenuButton(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] is_active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            disabled={disabled}
            class={class}
            id={id}
            attr:data-sidebar-menu-button=""
            attr:data-active={if is_active { "true" } else { "" }}
        >
            {children.map(|c| c())}
        </ButtonPrimitive>
    }
}

#[component]
pub fn SidebarInset(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div attr:data-sidebar-inset="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}
