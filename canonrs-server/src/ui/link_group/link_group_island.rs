//! @canon-level: strict
//! LinkGroup Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::link_group_ui::{LinkGroup, LinkGroupDirection};

#[island]
pub fn LinkGroupInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn LinkGroupIsland(
    children: Children,
    #[prop(optional)] label: Option<std::sync::Arc<dyn Fn() -> AnyView + Send + Sync>>,
    #[prop(default = LinkGroupDirection::Vertical)] direction: LinkGroupDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LinkGroupInit />
        <LinkGroup label=label.unwrap_or_else(|| std::sync::Arc::new(|| leptos::prelude::view! {}.into_any())) direction=direction class=class>
            {children()}
        </LinkGroup>
    }
}
