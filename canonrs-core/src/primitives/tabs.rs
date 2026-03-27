//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - data-rs-state SSR + behavior

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::infra::state_engine::{activity_attrs, disabled_attrs};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl TabsOrientation {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Horizontal => "horizontal", Self::Vertical => "vertical" }
    }
}

#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs=""
            data-rs-component="Tabs"
            data-rs-behavior="navigation"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListPrimitive(
    children: Children,
    #[prop(default = TabsOrientation::Horizontal)] orientation: TabsOrientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs-list=""
            role="tablist"
            aria-orientation=orientation.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = activity_attrs(active);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            role="tab"
            data-rs-tabs-trigger=""
            data-rs-value=value
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-selected=s.aria_selected
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContentPrimitive(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = activity_attrs(active);
    view! {
        <div
            data-rs-tabs-content=""
            data-rs-value=value
            data-rs-state=s.data_rs_state
            role="tabpanel"
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}
