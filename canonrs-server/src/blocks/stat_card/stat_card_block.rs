//! @canon-id: stat-card
//! @canon-type: block
//! @canon-category: data
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: icon, label, value, change
//! @canon-label: Stat Card
//! @canon-description: Metric stat display block
//! @canon-tags: stat-card, stat, metric, kpi, number, indicator
//! @canon-slot-accepts: icon=Any,label=Any,value=Any,change=Any
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
            data-rs-block=""
            data-rs-component="StatCard"
            data-rs-trend=trend.as_str()
            style=style
            class=class
        >
            {icon.map(|i| view! { <div data-rs-region="icon">{i()}</div> })}
            {label.map(|l| view! { <div data-rs-region="label">{l()}</div> })}
            {value.map(|v| view! { <div data-rs-region="value">{v()}</div> })}
            {change.map(|c| view! { <div data-rs-region="change">{c()}</div> })}
        </div>
    }
}
