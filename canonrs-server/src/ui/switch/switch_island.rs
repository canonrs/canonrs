//! @canon-level: strict
//! Switch Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::switch_ui::Switch;

#[component]
pub fn SwitchIsland(
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Switch checked=checked disabled=disabled name=name value=value class=class>
            <span></span>
        </Switch>
    }
}
