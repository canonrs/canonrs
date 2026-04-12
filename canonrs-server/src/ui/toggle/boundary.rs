//! @canon-level: strict
//! Toggle Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::toggle_ui::Toggle as ToggleUi;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToggleUi pressed=pressed aria_label=aria_label class=class>
            {children()
};
        </ToggleUi>
    }
}
