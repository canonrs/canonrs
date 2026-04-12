use leptos::prelude::*;
use super::context_menu_boundary::{
    ContextMenu, ContextMenuTrigger, ContextMenuContent,
    ContextMenuItem,
};

#[component]
pub fn ContextMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ContextMenu>
                    <ContextMenuTrigger>
                        <div style="padding: 2rem; border: 2px dashed currentColor; border-radius: 0.5rem; cursor: context-menu;">
                            "Right-click here"
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Edit"</ContextMenuItem>
                        <ContextMenuItem>"Duplicate"</ContextMenuItem>
                        <ContextMenuItem>"Delete"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Context menu appears at cursor position on right-click."
            </p>
        </div>
    }
}
