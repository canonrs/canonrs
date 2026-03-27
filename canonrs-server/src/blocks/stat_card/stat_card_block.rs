//! @canon-id: stat-card
//! @canon-type: block
//! @canon-category: data
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: icon, label, value, change
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum StatTrend { #[default] Neutral, Up, Down }
impl StatTrend {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Neutral => "neutral", Self::Up => "up", Self::Down => "down" }
    }
}

#[component]
pub fn StatCard(
    #[prop(default = StatTrend::Neutral)] trend: StatTrend,
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(optional)] label: Option<ChildrenFn>,
    #[prop(optional)] value: Option<ChildrenFn>,
    #[prop(optional)] change: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
) -> impl IntoView {
    view! {
        <div
            data-block="stat-card"
            data-block-version="1"
            style=style
            data-block-trend=trend.as_str()
            class=class
        >
            {icon.map(|i| view! { <div data-block-region="icon">{i()}</div> })}
            {label.map(|l| view! { <div data-block-region="label">{l()}</div> })}
            {value.map(|v| view! { <div data-block-region="value">{v()}</div> })}
            {change.map(|c| view! { <div data-block-region="change">{c()}</div> })}
        </div>
    }
}
