use crate::primitives::sidebar::*;
use leptos::prelude::*;

#[component]
pub fn SidebarProvider(
    children: ChildrenFn,
    #[prop(default = true)] default_open: bool,
    #[prop(default = false)] is_mobile: bool,
    #[prop(default = CollapsibleMode::Offcanvas)] collapsible: CollapsibleMode,
    #[prop(default = SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(default = SidebarSide::Left)] side: SidebarSide,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarProviderPrimitive
            default_open=default_open
            is_mobile=is_mobile
            collapsible=collapsible
            variant=variant
            side=side
        >
            <div class=format!("flex min-h-svh w-full {}", class)>
                {children()}
            </div>
        </SidebarProviderPrimitive>
    }
}

#[component]
pub fn Sidebar(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <aside class=format!("w-64 border-r bg-sidebar text-sidebar-foreground {}", class)>
            <SidebarPrimitive>
                {children()}
            </SidebarPrimitive>
        </aside>
    }
}

#[component]
pub fn SidebarHeader(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarHeaderPrimitive>
            <div class=format!("flex flex-col gap-2 p-4 {}", class)>{children()}</div>
        </SidebarHeaderPrimitive>
    }
}

#[component]
pub fn SidebarContent(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarContentPrimitive>
            <div class=format!("flex flex-1 flex-col gap-2 overflow-y-auto p-2 {}", class)>{children()}</div>
        </SidebarContentPrimitive>
    }
}

#[component]
pub fn SidebarFooter(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarFooterPrimitive>
            <div class=format!("flex flex-col gap-2 p-4 {}", class)>{children()}</div>
        </SidebarFooterPrimitive>
    }
}

#[component]
pub fn SidebarGroup(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupPrimitive>
            <div class=format!("flex flex-col gap-2 {}", class)>{children()}</div>
        </SidebarGroupPrimitive>
    }
}

#[component]
pub fn SidebarGroupLabel(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupLabelPrimitive>
            <div class=format!("px-2 py-1 text-xs font-semibold text-sidebar-foreground/70 {}", class)>{children()}</div>
        </SidebarGroupLabelPrimitive>
    }
}

#[component]
pub fn SidebarGroupContent(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupContentPrimitive>
            <div class=format!("flex flex-col {}", class)>{children()}</div>
        </SidebarGroupContentPrimitive>
    }
}

#[component]
pub fn SidebarMenu(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuPrimitive>
            <ul class=format!("flex flex-col gap-1 {}", class)>{children()}</ul>
        </SidebarMenuPrimitive>
    }
}

#[component]
pub fn SidebarMenuItem(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuItemPrimitive>
            <li class=format!("{}", class)>{children()}</li>
        </SidebarMenuItemPrimitive>
    }
}

#[component]
pub fn SidebarMenuButton(
    children: ChildrenFn,
    #[prop(default = false)] is_active: bool,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = Callback::new(|_| {}))] on_click: Callback<()>,
) -> impl IntoView {
    let base = "flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-sidebar-accent hover:text-sidebar-accent-foreground cursor-pointer";
    let active = if is_active { "bg-sidebar-accent text-sidebar-accent-foreground font-medium" } else { "" };
    
    view! {
        <SidebarMenuButtonPrimitive is_active=is_active>
            <span 
                class=format!("{} {} {}", base, active, class)
                on:click=move |_| on_click.run(())
            >
                {children()}
            </span>
        </SidebarMenuButtonPrimitive>
    }
}

#[component]
pub fn SidebarMenuSub(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuSubPrimitive>
            <ul class=format!("ml-4 flex flex-col gap-1 border-l pl-2 {}", class)>{children()}</ul>
        </SidebarMenuSubPrimitive>
    }
}

#[component]
pub fn SidebarMenuSubItem(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuSubItemPrimitive>
            <li class=format!("{}", class)>{children()}</li>
        </SidebarMenuSubItemPrimitive>
    }
}

#[component]
pub fn SidebarTrigger(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <button 
            class=format!("inline-flex items-center justify-center rounded-md p-2 hover:bg-accent cursor-pointer {}", class)
            on:click=move |_| ctx.toggle()
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 15 15" fill="none" class="h-4 w-4">
                <path d="M1.5 3C1.22386 3 1 3.22386 1 3.5C1 3.77614 1.22386 4 1.5 4H13.5C13.7761 4 14 3.77614 14 3.5C14 3.22386 13.7761 3 13.5 3H1.5ZM1 7.5C1 7.22386 1.22386 7 1.5 7H13.5C13.7761 7 14 7.22386 14 7.5C14 7.77614 13.7761 8 13.5 8H1.5C1.22386 8 1 7.77614 1 7.5ZM1 11.5C1 11.2239 1.22386 11 1.5 11H13.5C13.7761 11 14 11.2239 14 11.5C14 11.7761 13.7761 12 13.5 12H1.5C1.22386 12 1 11.7761 1 11.5Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"/>
            </svg>
        </button>
    }
}

#[component]
pub fn SidebarInset(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SidebarInsetPrimitive>
            <main class=format!("flex flex-1 flex-col {}", class)>{children()}</main>
        </SidebarInsetPrimitive>
    }
}
