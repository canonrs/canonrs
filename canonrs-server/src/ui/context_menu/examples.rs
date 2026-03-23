use leptos::prelude::*;
use super::context_menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <ContextMenu id="context-menu-ex".to_string()>
                <ContextMenuTrigger>
                    <div class="p-4 border rounded">"Right-click me"</div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Copy"</ContextMenuItem>
                    <ContextMenuItem>"Paste"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuItem>"Delete"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        </div>
    }
}
