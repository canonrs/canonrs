use leptos::prelude::*;
use super::context_menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger>
                <div style="padding:2rem;border:1px dashed currentColor;border-radius:0.5rem;cursor:context-menu;">"Right-click here"</div>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem>"Edit"</ContextMenuItem>
                <ContextMenuItem>"Copy"</ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem>"Delete"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
}
