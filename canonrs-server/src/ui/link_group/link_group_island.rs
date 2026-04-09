//! @canon-level: strict
//! LinkGroup Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::link_group_ui::{LinkGroup, LinkGroupDirection};



#[component]
pub fn LinkGroupIsland(
    children: Children,
    #[prop(optional)] label: Option<std::sync::Arc<dyn Fn() -> AnyView + Send + Sync>>,
    #[prop(default = LinkGroupDirection::Vertical)] direction: LinkGroupDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LinkGroup label=label.unwrap_or_else(|| std::sync::Arc::new(|| leptos::prelude::view! {}.into_any())) direction=direction class=class>
            {children()}
        </LinkGroup>
    }
}
