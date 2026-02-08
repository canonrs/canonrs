use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct SidebarContext {
    pub open: RwSignal<bool>,
    pub collapsed: RwSignal<bool>,
}

impl SidebarContext {
    pub fn toggle_collapsed(&self) {
        self.collapsed.set(!self.collapsed.get());
    }
    
    pub fn toggle_open(&self) {
        self.open.set(!self.open.get());
    }
}

#[component]
pub fn SidebarProvider(
    #[prop(default = true)] initial_open: bool,
    #[prop(default = false)] initial_collapsed: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(initial_open);
    let collapsed = RwSignal::new(initial_collapsed);
    
    provide_context(SidebarContext { open, collapsed });
    
    view! { {children()} }
}

pub fn use_sidebar() -> SidebarContext {
    expect_context::<SidebarContext>()
}
