// Adicione este import no topo do sidebar.rs existente
use crate::ui::sidebar_mobile::SidebarMobile;

// Substitua a implementação do Sidebar component (linha ~60)
#[component]
pub fn Sidebar(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();

    let base_classes = StoredValue::new(
        "group peer text-sidebar-foreground hidden md:block".to_string()
    );

    let container_classes = StoredValue::new(format!(
        "fixed inset-y-0 z-10 hidden h-svh w-[var(--sidebar-width)] transition-[left,right,width] duration-200 ease-linear md:flex {}",
        if ctx.side == SidebarSide::Left { "left-0" } else { "right-0" },
        class
    ));

    let inner_classes = StoredValue::new(
        "bg-sidebar flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-sm".to_string()
    );

    // Mobile: usa Dialog como Sheet
    if ctx.is_mobile {
        return view! {
            <SidebarMobile>
                {children()}
            </SidebarMobile>
        };
    }

    // Desktop
    view! {
        <div
            class=base_classes.get_value()
            data-state=move || match ctx.state.get() {
                SidebarState::Expanded => "expanded",
                SidebarState::Collapsed => "collapsed",
            }
            data-collapsible=move || match ctx.collapsible {
                CollapsibleMode::Offcanvas => "offcanvas",
                CollapsibleMode::Icon => "icon",
                CollapsibleMode::None => "none",
            }
            data-variant=move || match ctx.variant {
                SidebarVariant::Sidebar => "sidebar",
                SidebarVariant::Floating => "floating",
                SidebarVariant::Inset => "inset",
            }
            data-side=move || match ctx.side {
                SidebarSide::Left => "left",
                SidebarSide::Right => "right",
            }
        >
            <div class="relative w-[var(--sidebar-width)] bg-transparent transition-[width] duration-200 ease-linear group-data-[collapsible=offcanvas]:w-0 group-data-[collapsible=icon]:w-[var(--sidebar-width-icon)]" />
            
            <div class=container_classes.get_value()>
                <SidebarPrimitive>
                    <div class=inner_classes.get_value()>
                        {children()}
                    </div>
                </SidebarPrimitive>
            </div>
        </div>
    }
}
