//! @canon-level: strict
//! Tabs Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-init

use leptos::prelude::*;
use super::tabs_ui::{Tabs, TabsTrigger, TabsContent};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn TabsRootIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Tabs class=class>{children()}</Tabs> }
}

#[component]
pub fn TabsTriggerIsland(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let dis = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! { <TabsTrigger value=value active=ActivityState::Inactive disabled=dis>{children()}</TabsTrigger> }
}

#[component]
pub fn TabsContentIsland(
    children: Children,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! { <TabsContent value=value active=ActivityState::Inactive>{children()}</TabsContent> }
}
