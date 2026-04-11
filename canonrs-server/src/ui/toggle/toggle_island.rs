//! @canon-level: strict
//! Toggle Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::toggle_ui::Toggle;

#[component]
pub fn ToggleIsland(
    children: Children,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Toggle pressed=pressed aria_label=aria_label class=class>
            {children()}
        </Toggle>
    }
}
