#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    StatSize, StatTrend, StatAlign,
    StatPrimitive, StatValuePrimitive, StatLabelPrimitive,
    StatDeltaPrimitive, StatIconPrimitive, StatHeaderPrimitive, StatBodyPrimitive
};
use canonrs_core::LoadingState;

#[component]
pub fn Stat(
    children: Children,
    #[prop(default = StatSize::Md)] size: StatSize,
    #[prop(default = StatAlign::Start)] align: StatAlign,
    #[prop(optional)] trend: Option<StatTrend>,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatPrimitive class={class}>
            <div
                data-rs-stat-wrapper=""
                data-rs-size={size.as_str()}
                data-rs-align={align.as_str()}
                data-rs-trend={trend.map(|t| t.as_str())}
                data-rs-loading={if loading == LoadingState::Loading { Some("") } else { None }}
                aria-busy={if loading == LoadingState::Loading { Some("true") } else { None }}
                role="group"
            >
                {children()}
            </div>
        </StatPrimitive>
    }
}

#[component]
pub fn StatHeader(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatHeaderPrimitive class={class}>
            {children()}
        </StatHeaderPrimitive>
    }
}

#[component]
pub fn StatBody(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatBodyPrimitive class={class}>
            {children()}
        </StatBodyPrimitive>
    }
}

#[component]
pub fn StatValue(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatValuePrimitive class={class}>
            {children()}
        </StatValuePrimitive>
    }
}

#[component]
pub fn StatLabel(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatLabelPrimitive class={class}>
            {children()}
        </StatLabelPrimitive>
    }
}

#[component]
pub fn StatDelta(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatDeltaPrimitive class={class}>
            {children()}
        </StatDeltaPrimitive>
    }
}

#[component]
pub fn StatIcon(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatIconPrimitive class={class}>
            {children()}
        </StatIconPrimitive>
    }
}

#[component]
pub fn StatPreview() -> impl IntoView {
    view! { <Stat size=StatSize::Md>"1,234"</Stat> }
}
