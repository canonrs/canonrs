//! Stat Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::stat_ui::{Stat, StatHeader, StatBody, StatValue, StatLabel, StatDelta, StatIcon, StatSize, StatAlign, StatTrend};
use canonrs_core::meta::LoadingState;

#[component]
pub fn StatIsland(
    children: Children,
    #[prop(default = StatSize::Md)] size:          StatSize,
    #[prop(default = StatAlign::Start)] align:     StatAlign,
    #[prop(default = StatTrend::Neutral)] trend:   StatTrend,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! { <Stat size=size align=align trend=trend loading=loading class=class>{children()}</Stat> }
}

#[component]
pub fn StatHeaderIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatHeader class=class>{children()}</StatHeader> }
}

#[component]
pub fn StatBodyIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatBody class=class>{children()}</StatBody> }
}

#[component]
pub fn StatValueIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatValue class=class>{children()}</StatValue> }
}

#[component]
pub fn StatLabelIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatLabel class=class>{children()}</StatLabel> }
}

#[component]
pub fn StatDeltaIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatDelta class=class>{children()}</StatDelta> }
}

#[component]
pub fn StatIconIsland(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatIcon class=class>{children()}</StatIcon> }
}
