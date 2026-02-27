//! # StatCard Block
use leptos::prelude::*;

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] change: Option<String>,
    #[prop(optional, into)] trend: Option<String>,
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-stat-card {}", class)
            data-block="stat-card"
            data-block-version="1"
        >
            <div data-block-region="header" class="canon-stat-card__header">
                <span data-block-region="label" class="canon-stat-card__label">{label}</span>
                <div data-block-region="icon">
                    {icon.map(|i| view! { <span class="canon-stat-card__icon">{i()}</span> })}
                </div>
            </div>
            <div data-block-region="value" class="canon-stat-card__value">{value}</div>
            <div data-block-region="change">
                {change.map(|c| {
                    let trend_class = trend.as_ref().map(|t| format!("canon-stat-card__change--{}", t)).unwrap_or_default();
                    view! { <div class=format!("canon-stat-card__change {}", trend_class)>{c}</div> }
                })}
            </div>
        </div>
    }
}
