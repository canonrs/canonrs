use crate::tokens::{SEMANTIC, SPACING};
use crate::ui::SidebarTrigger;
use leptos::prelude::*;

#[component]
pub fn Header(
    children: Children,
    #[prop(default = false)] show_sidebar_trigger: bool,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let header_class = format!(
        "flex h-16 shrink-0 items-center gap-4 border-b px-[{}] {}",
        SPACING.lg, class
    );

    view! {
        <header
            class=header_class
            style=format!("background: {}; border-color: {};", SEMANTIC.background, SEMANTIC.border)
        >
            {show_sidebar_trigger.then(|| view! {
                <SidebarTrigger class="-ml-1".to_string() />
            })}
            {children()}
        </header>
    }
}
