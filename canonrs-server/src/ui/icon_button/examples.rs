use leptos::prelude::*;
use super::{IconButton, IconButtonVariant};
use crate::ui::icon::IconSize;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Sizes
            <div>
                <h4>"Sizes"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <IconButton size=IconSize::Sm aria_label="Edit (small)".to_string()>"✏️"</IconButton>
                    <IconButton size=IconSize::Md aria_label="Edit (medium)".to_string()>"✏️"</IconButton>
                    <IconButton size=IconSize::Lg aria_label="Edit (large)".to_string()>"✏️"</IconButton>
                </div>
            </div>

            // Variants
            <div>
                <h4>"Variants"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <IconButton variant=IconButtonVariant::Default aria_label="Settings".to_string()>"⚙️"</IconButton>
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Search".to_string()>"🔍"</IconButton>
                    <IconButton variant=IconButtonVariant::Outline aria_label="More".to_string()>"⋯"</IconButton>
                    <IconButton variant=IconButtonVariant::Solid aria_label="Add".to_string()>"+"</IconButton>
                    <IconButton variant=IconButtonVariant::Destructive aria_label="Delete".to_string()>"🗑️"</IconButton>
                </div>
            </div>

            // States
            <div>
                <h4>"States"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <IconButton aria_label="Normal".to_string()>"👍"</IconButton>
                    <IconButton disabled=true aria_label="Disabled".to_string()>"👍"</IconButton>
                    <IconButton loading=true aria_label="Loading".to_string()>"👍"</IconButton>
                </div>
            </div>

            // Action toolbar example
            <div>
                <h4>"Toolbar Actions"</h4>
                <div style="display: flex; gap: var(--space-xs); padding: var(--space-sm); background: var(--theme-surface-muted); border-radius: var(--radius-md); width: fit-content;">
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Bold".to_string()>"B"</IconButton>
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Italic".to_string()>"I"</IconButton>
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Underline".to_string()>"U"</IconButton>
                    <div style="width: 1px; height: 24px; background: var(--theme-surface-border); margin: 0 var(--space-xs);"></div>
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Link".to_string()>"🔗"</IconButton>
                    <IconButton variant=IconButtonVariant::Ghost aria_label="Image".to_string()>"🖼️"</IconButton>
                </div>
            </div>

            // Combined example
            <div>
                <h4>"Combined Features"</h4>
                <div style="display: flex; gap: 1rem;">
                    <IconButton 
                        size=IconSize::Lg 
                        variant=IconButtonVariant::Solid 
                        aria_label="Create new item".to_string()
                    >"+"</IconButton>
                    <IconButton 
                        size=IconSize::Lg 
                        variant=IconButtonVariant::Destructive 
                        aria_label="Delete selected items".to_string()
                    >"🗑️"</IconButton>
                </div>
            </div>
        </div>
    }
}
