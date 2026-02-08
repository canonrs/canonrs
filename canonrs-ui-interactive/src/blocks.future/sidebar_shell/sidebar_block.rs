//! # Sidebar Block
//! **Canon Rule:** Block usa Children simples

use leptos::prelude::*;
use crate::ui::sidebar::{Sidebar, SidebarContent};

#[component]
pub fn SidebarBlock(
    children: Children,
    open: Signal<bool>,
    collapsed: Signal<bool>,
) -> impl IntoView {
    view! {
        <Sidebar open=open collapsed=collapsed>
            <SidebarContent>
                {children()}
            </SidebarContent>
        </Sidebar>
    }
}
