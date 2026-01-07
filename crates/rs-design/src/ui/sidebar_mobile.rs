// TEMPORARILY DISABLED - Type inference issues to be resolved

use leptos::prelude::*;

#[component]
pub fn SidebarMobile(
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class="p-4 text-sm">
            "Mobile Sidebar (under construction)"
            {children()}
        </div>
    }
}
