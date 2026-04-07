use leptos::prelude::*;
use super::stat_ui::{
    Stat, StatHeader, StatBody, StatValue, StatLabel, StatDelta, StatIcon,
    StatSize, StatAlign, StatTrend,
};
use canonrs_core::LoadingState;

#[component]
pub fn StatIsland(
    children: Children,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] align: Option<String>,
    #[prop(optional, into)] trend: Option<String>,
    #[prop(optional, into)] loading: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size_val = match size.as_deref() {
        Some("sm") => StatSize::Sm,
        Some("lg") => StatSize::Lg,
        _          => StatSize::Md,
    };
    let align_val = match align.as_deref() {
        Some("center") => StatAlign::Center,
        Some("end")    => StatAlign::End,
        _              => StatAlign::Start,
    };
    let loading_val = match loading.as_deref() {
        Some("loading") => LoadingState::Loading,
        _               => LoadingState::Idle,
    };
    let class_val = class.unwrap_or_default();

    match trend.as_deref() {
        Some("increase") => view! {
            <Stat size=size_val align=align_val trend=StatTrend::Increase loading=loading_val class=class_val>
                {children()}
            </Stat>
        }.into_any(),
        Some("decrease") => view! {
            <Stat size=size_val align=align_val trend=StatTrend::Decrease loading=loading_val class=class_val>
                {children()}
            </Stat>
        }.into_any(),
        Some("neutral") => view! {
            <Stat size=size_val align=align_val trend=StatTrend::Neutral loading=loading_val class=class_val>
                {children()}
            </Stat>
        }.into_any(),
        _ => view! {
            <Stat size=size_val align=align_val loading=loading_val class=class_val>
                {children()}
            </Stat>
        }.into_any(),
    }
}

#[component]
pub fn StatHeaderIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatHeader class=class.unwrap_or_default()>{children()}</StatHeader> }
}

#[component]
pub fn StatBodyIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatBody class=class.unwrap_or_default()>{children()}</StatBody> }
}

#[component]
pub fn StatValueIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatValue class=class.unwrap_or_default()>{children()}</StatValue> }
}

#[component]
pub fn StatLabelIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatLabel class=class.unwrap_or_default()>{children()}</StatLabel> }
}

#[component]
pub fn StatDeltaIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatDelta class=class.unwrap_or_default()>{children()}</StatDelta> }
}

#[component]
pub fn StatIconIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <StatIcon class=class.unwrap_or_default()>{children()}</StatIcon> }
}
