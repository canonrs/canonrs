use leptos::prelude::*;
use canonrs_core::primitives::{
    StatPrimitive, StatValuePrimitive, StatLabelPrimitive,
    StatDeltaPrimitive, StatIconPrimitive, StatHeaderPrimitive, StatBodyPrimitive
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatSize { Sm, Md, Lg }
impl StatSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sm => "sm", Self::Md => "md", Self::Lg => "lg" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatTrend { Increase, Decrease, Neutral }
impl StatTrend {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Increase => "increase", Self::Decrease => "decrease", Self::Neutral => "neutral" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatAlign { Start, Center, End }
impl StatAlign {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Start => "start", Self::Center => "center", Self::End => "end" }
    }
}

#[component]
pub fn Stat(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = StatSize::Md)] size: StatSize,
    #[prop(default = StatAlign::Start)] align: StatAlign,
    #[prop(optional)] trend: Option<StatTrend>,
    #[prop(default = false)] loading: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatPrimitive class={class}>
            <div
                data-rs-stat-wrapper=""
                data-rs-size={size.as_str()}
                data-rs-align={align.as_str()}
                data-rs-trend={trend.map(|t| t.as_str())}
                data-rs-loading={loading.then_some("")}
                aria-busy={loading.then_some("true")}
                role="group"
            >
                {children.map(|c| c())}
            </div>
        </StatPrimitive>
    }
}

#[component]
pub fn StatHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatHeaderPrimitive class={class}>
            {children.map(|c| c())}
        </StatHeaderPrimitive>
    }
}

#[component]
pub fn StatBody(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatBodyPrimitive class={class}>
            {children.map(|c| c())}
        </StatBodyPrimitive>
    }
}

#[component]
pub fn StatValue(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatValuePrimitive class={class}>
            {children.map(|c| c())}
        </StatValuePrimitive>
    }
}

#[component]
pub fn StatLabel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatLabelPrimitive class={class}>
            {children.map(|c| c())}
        </StatLabelPrimitive>
    }
}

#[component]
pub fn StatDelta(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatDeltaPrimitive class={class}>
            {children.map(|c| c())}
        </StatDeltaPrimitive>
    }
}

#[component]
pub fn StatIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatIconPrimitive class={class}>
            {children.map(|c| c())}
        </StatIconPrimitive>
    }
}

#[component]
pub fn StatPreview() -> impl IntoView {
    view! { <Stat size=StatSize::Md>"1,234"</Stat> }
}
