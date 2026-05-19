#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::typography::{
    TextPrimitive, TextSize, TextWeight, TextVariant,
    HeadingPrimitive, HeadingLevel,
    CaptionPrimitive,
    MetricPrimitive,
    CodeInlinePrimitive,
};

#[component]
pub fn Text(
    children: Children,
    #[prop(default = TextSize::Md)]       size:    TextSize,
    #[prop(default = TextWeight::Normal)] weight:  TextWeight,
    #[prop(default = TextVariant::Default)] variant: TextVariant,
    #[prop(default = false)] mono: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TextPrimitive size=size weight=weight variant=variant mono=mono class=class>{children()}</TextPrimitive> }
}

#[component]
pub fn Heading(
    children: Children,
    #[prop(default = HeadingLevel::H2)] level:  HeadingLevel,
    #[prop(default = TextSize::Lg)]     size:   TextSize,
    #[prop(default = TextWeight::Bold)] weight: TextWeight,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <HeadingPrimitive level=level size=size weight=weight class=class>{children()}</HeadingPrimitive> }
}

#[component]
pub fn Caption(
    children: Children,
    #[prop(default = TextVariant::Muted)] variant: TextVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CaptionPrimitive variant=variant class=class>{children()}</CaptionPrimitive> }
}

#[component]
pub fn Metric(
    children: Children,
    #[prop(default = TextSize::Xl)]       size:    TextSize,
    #[prop(default = TextWeight::Bold)]   weight:  TextWeight,
    #[prop(default = TextVariant::Default)] variant: TextVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <MetricPrimitive size=size weight=weight variant=variant class=class>{children()}</MetricPrimitive> }
}

#[component]
pub fn CodeInline(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CodeInlinePrimitive class=class>{children()}</CodeInlinePrimitive> }
}
