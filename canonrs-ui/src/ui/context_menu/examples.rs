use leptos::prelude::*;
use super::context_menu_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <ContextMenu open=Signal::from(false) id="context-menu-ex".to_string()>
                <ContextMenuTrigger controls_id="context-menu-content-ex".to_string()>
                    <div class="p-4 border rounded">"Right-click me"</div>
                </ContextMenuTrigger>
                <ContextMenuContent content_id="context-menu-content-ex".to_string()>
                    <ContextMenuItem>"Cut"</ContextMenuItem>
                    <ContextMenuItem>"Copy"</ContextMenuItem>
                    <ContextMenuItem>"Paste"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuItem>"Delete"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        </div>
    }
}
