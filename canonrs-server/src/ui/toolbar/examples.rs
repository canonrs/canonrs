use leptos::prelude::*;
use super::{Toolbar, ToolbarSeparator};
use crate::ui::button::Button;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Basic toolbar
            <div>
                <h4>"Basic Toolbar"</h4>
                <Toolbar aria_label="Document actions">
                    <Button>"New"</Button>
                    <Button>"Save"</Button>
                    <Button>"Print"</Button>
                </Toolbar>
            </div>

            // With separator
            <div>
                <h4>"Toolbar with Separators"</h4>
                <Toolbar aria_label="Text formatting">
                    <Button>"Cut"</Button>
                    <Button>"Copy"</Button>
                    <Button>"Paste"</Button>
                    <ToolbarSeparator />
                    <Button>"Bold"</Button>
                    <Button>"Italic"</Button>
                    <Button>"Underline"</Button>
                    <ToolbarSeparator />
                    <Button>"Undo"</Button>
                    <Button>"Redo"</Button>
                </Toolbar>
            </div>
        </div>
    }
}
