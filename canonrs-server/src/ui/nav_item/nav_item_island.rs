//! @canon-level: strict
//! NavItem Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::nav_item_ui::NavItem;

#[component]
pub fn NavItemIsland(
    #[prop(into)] label: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <NavItem label=label href=href active=active.into() disabled=disabled.into() class=class /> }
}
