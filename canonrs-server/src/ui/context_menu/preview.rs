use leptos::prelude::*;
use super::context_menu_ui::{
    ContextMenu, ContextMenuTrigger, ContextMenuContent,
    ContextMenuItem, ContextMenuSeparator,
};

#[component]
pub fn ContextMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ContextMenu>
                    <ContextMenuTrigger>
                        <div style="padding:2rem;border:1px dashed currentColor;border-radius:var(--radius-md);text-align:center;cursor:context-menu;">
                            "Right-click here"
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"View"</ContextMenuItem>
                        <ContextMenuItem>"Edit"</ContextMenuItem>
                        <ContextMenuItem>"Duplicate"</ContextMenuItem>
                        <ContextMenuSeparator />
                        <ContextMenuItem>"Delete"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Context menu interaction and roles enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With groups"</span>
                <div data-rs-showcase-preview-row="">
                    <ContextMenu>
                        <ContextMenuTrigger>
                            <div style="padding:2rem;border:1px dashed currentColor;border-radius:var(--radius-md);text-align:center;cursor:context-menu;">
                                "Right-click for file actions"
                            </div>
                        </ContextMenuTrigger>
                        <ContextMenuContent>
                            <ContextMenuItem>"Open"</ContextMenuItem>
                            <ContextMenuItem>"Open with..."</ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem>"Cut"</ContextMenuItem>
                            <ContextMenuItem>"Copy"</ContextMenuItem>
                            <ContextMenuItem>"Paste"</ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem>"Rename"</ContextMenuItem>
                            <ContextMenuItem>"Move to trash"</ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenu>
                </div>
            </div>
        </div>
    }
}
