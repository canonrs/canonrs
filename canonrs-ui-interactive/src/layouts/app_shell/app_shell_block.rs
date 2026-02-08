//! # AppShell Block
//! **Canon Rule:** Slots explícitos sidebar + inset

use leptos::prelude::*;
use crate::ui::sidebar::Sidebar;

#[component]
pub fn AppShellBlock(
    #[prop(into)] sidebar: ViewFn,
    #[prop(into)] inset: ViewFn,
    open: Signal<bool>,
    collapsed: Signal<bool>,
) -> impl IntoView {
    view! {
        <Sidebar open=open collapsed=collapsed>
            {sidebar.run()}
            {inset.run()}
        </Sidebar>
    }
}
