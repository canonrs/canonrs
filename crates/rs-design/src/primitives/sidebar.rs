//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

// ═══════════════════════════════════════════════════════════
// TYPES & CONTEXT
// ═══════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollapsibleMode {
    Offcanvas,
    Icon,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SidebarVariant {
    Sidebar,
    Floating,
    Inset,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SidebarSide {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct SidebarContext {
    pub open: RwSignal<bool>,
    pub is_mobile: bool,
    pub collapsible: CollapsibleMode,
    pub variant: SidebarVariant,
    pub side: SidebarSide,
}

impl SidebarContext {
    pub fn new(
        default_open: bool,
        is_mobile: bool,
        collapsible: CollapsibleMode,
        variant: SidebarVariant,
        side: SidebarSide,
    ) -> Self {
        Self {
            open: RwSignal::new(default_open),
            is_mobile,
            collapsible,
            variant,
            side,
        }
    }

    pub fn toggle(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::console;
            console::log_1(&format!("🔄 Toggle called! is_mobile: {}, open before: {}", self.is_mobile, self.open.get()).into());
        }
        
        self.open.update(|open| *open = !*open);
        
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::console;
            console::log_1(&format!("✅ Open after: {}", self.open.get()).into());
        }
    }
}

// ═══════════════════════════════════════════════════════════
// PRIMITIVES
// ═══════════════════════════════════════════════════════════

#[component]
pub fn SidebarProviderPrimitive(
    children: Children,
    #[prop(default = true)] default_open: bool,
    #[prop(default = false)] is_mobile: bool,
    #[prop(default = CollapsibleMode::Offcanvas)] collapsible: CollapsibleMode,
    #[prop(default = SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(default = SidebarSide::Left)] side: SidebarSide,
) -> impl IntoView {
    let ctx = SidebarContext::new(default_open, is_mobile, collapsible, variant, side);
    provide_context(ctx);
    
    view! {
        <div data-sidebar-provider="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarPrimitive(
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <aside 
            data-sidebar=""
            class:w-64=move || ctx.open.get()
            class:w-0=move || !ctx.open.get()
            class="transition-all duration-300 overflow-hidden"
        >
            {children()}
        </aside>
    }
}

#[component]
pub fn SidebarHeaderPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-header="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarContentPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-content="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooterPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-footer="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-group="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupLabelPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-group-label="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupContentPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-group-content="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuPrimitive(children: Children) -> impl IntoView {
    view! {
        <nav data-sidebar-menu="">
            {children()}
        </nav>
    }
}

#[component]
pub fn SidebarMenuItemPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-menu-item="">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuButtonPrimitive(
    children: Children,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        <button
            data-sidebar-menu-button=""
            data-active=is_active
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SidebarMenuSubPrimitive(children: Children) -> impl IntoView {
    view! {
        <ul data-sidebar-menu-sub="">
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuSubItemPrimitive(children: Children) -> impl IntoView {
    view! {
        <li data-sidebar-menu-sub-item="">
            {children()}
        </li>
    }
}

#[component]
pub fn SidebarInsetPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-inset="">
            {children()}
        </div>
    }
}
