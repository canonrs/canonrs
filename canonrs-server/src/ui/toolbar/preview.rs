use leptos::prelude::*;
use super::toolbar_ui::{Toolbar, ToolbarSeparator};
use super::ToolbarOrientation;
use crate::ui::toggle::toggle_ui::Toggle;

#[component]
pub fn ToolbarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <Toolbar aria_label="Text formatting" orientation=ToolbarOrientation::Horizontal>
                    <Toggle aria_label="Bold">"B"</Toggle>
                    <Toggle aria_label="Italic" pressed=true>"I"</Toggle>
                    <Toggle aria_label="Underline">"U"</Toggle>
                    <ToolbarSeparator />
                    <Toggle aria_label="Align left">"←"</Toggle>
                    <Toggle aria_label="Align center">"↔"</Toggle>
                    <Toggle aria_label="Align right">"→"</Toggle>
                    <ToolbarSeparator />
                    <Toggle aria_label="Link">"🔗"</Toggle>
                </Toolbar>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toolbar role and orientation enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="">
                    <Toolbar aria_label="Actions" orientation=ToolbarOrientation::Vertical>
                        <Toggle aria_label="Cut">"✂"</Toggle>
                        <Toggle aria_label="Copy">"⎘"</Toggle>
                        <Toggle aria_label="Paste">"📋"</Toggle>
                        <ToolbarSeparator />
                        <Toggle aria_label="Delete">"🗑"</Toggle>
                    </Toolbar>
                </div>
            </div>
        </div>
    }
}
