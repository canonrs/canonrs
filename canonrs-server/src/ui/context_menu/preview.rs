use leptos::prelude::*;
use super::context_menu_boundary::{
    ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ContextMenuShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <ContextMenu>
                <ContextMenuTrigger>
                    <div data-rs-context-menu-target="">"Right-click here"</div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Edit"</ContextMenuItem>
                    <ContextMenuItem>"Duplicate"</ContextMenuItem>
                    <ContextMenuItem>"Delete"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
            <p data-rs-showcase-preview-anchor="">
                "Context menu appears at cursor position on right-click."
            </p>
        </Stack>
    }
}
