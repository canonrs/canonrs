use leptos::prelude::*;
use super::context_menu_island::{
    ContextMenuIsland, ContextMenuTriggerIsland, ContextMenuContentIsland,
    ContextMenuItemIsland,
};

#[component]
pub fn ContextMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ContextMenuIsland>
                    <ContextMenuTriggerIsland>
                        <div style="padding: 2rem; border: 2px dashed currentColor; border-radius: 0.5rem; cursor: context-menu;">
                            "Right-click here"
                        </div>
                    </ContextMenuTriggerIsland>
                    <ContextMenuContentIsland>
                        <ContextMenuItemIsland>"Edit"</ContextMenuItemIsland>
                        <ContextMenuItemIsland>"Duplicate"</ContextMenuItemIsland>
                        <ContextMenuItemIsland>"Delete"</ContextMenuItemIsland>
                    </ContextMenuContentIsland>
                </ContextMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Context menu appears at cursor position on right-click."
            </p>
        </div>
    }
}
