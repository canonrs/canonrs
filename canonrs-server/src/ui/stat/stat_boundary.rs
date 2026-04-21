//! Stat Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::stat_ui::{
    Stat as StatUi,
    StatHeader as StatHeaderUi,
    StatBody as StatBodyUi,
    StatValue as StatValueUi,
    StatLabel as StatLabelUi,
    StatDelta as StatDeltaUi,
    StatIcon as StatIconUi,
};
pub use canonrs_core::primitives::{StatSize, StatAlign, StatTrend};
use canonrs_core::meta::LoadingState;

#[component]
pub fn Stat(
    children: Children,
    #[prop(default = StatSize::Md)] size:          StatSize,
    #[prop(default = StatAlign::Start)] align:     StatAlign,
    #[prop(default = StatTrend::Neutral)] trend:   StatTrend,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! { <StatUi size=size align=align trend=trend loading=loading class=class>{children()}</StatUi> }
}

#[component]
pub fn StatHeader(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatHeaderUi class=class>{children()}</StatHeaderUi> }
}

#[component]
pub fn StatBody(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatBodyUi class=class>{children()}</StatBodyUi> }
}

#[component]
pub fn StatValue(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatValueUi class=class>{children()}</StatValueUi> }
}

#[component]
pub fn StatLabel(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatLabelUi class=class>{children()}</StatLabelUi> }
}

#[component]
pub fn StatDelta(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatDeltaUi class=class>{children()}</StatDeltaUi> }
}

#[component]
pub fn StatIcon(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <StatIconUi class=class>{children()}</StatIconUi> }
}
