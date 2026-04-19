//! @canon-level: strict
//! Tabs Boundary — Canon Rule #341 (zero-logic boundary)
//! CR-342 v4.0.0: interaction delegated to canonrs-interactions-nav

use leptos::prelude::*;
use super::tabs_ui::{Tabs, TabsList, TabsTrigger as TabsTriggerUi, TabsContent as TabsContentUi};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn TabsRoot(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] default_value: String,
) -> impl IntoView {
    view! { <Tabs class=class default_value=default_value>{children()}</Tabs> }
}

#[component]
pub fn TabsListBoundary(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TabsList class=class>{children()}</TabsList> }
}

#[component]
pub fn TabsTrigger(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let dis = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! { <TabsTriggerUi value=value active=active disabled=dis>{children()}</TabsTriggerUi> }
}

#[component]
pub fn TabsContent(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
) -> impl IntoView {
    view! { <TabsContentUi value=value active=active>{children()}</TabsContentUi> }
}
