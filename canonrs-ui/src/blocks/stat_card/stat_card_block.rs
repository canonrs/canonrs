//! # StatCard Block
//! Statistical data display card

use leptos::prelude::*;

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] change: Option<String>,
    #[prop(optional, into)] trend: Option<String>, // "up", "down", "neutral"
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div 
            class=format!("canon-stat-card {}", class)
            attr:data-block="stat-card"
        >
            <div class="canon-stat-card__header">
                <span class="canon-stat-card__label">{label}</span>
                {icon.map(|i| view! {
                    <span class="canon-stat-card__icon">{i()}</span>
                })}
            </div>
            
            <div class="canon-stat-card__value">{value}</div>
            
            {change.map(|c| {
                let trend_class = trend.as_ref().map(|t| format!("canon-stat-card__change--{}", t)).unwrap_or_default();
                view! {
                    <div class=format!("canon-stat-card__change {}", trend_class)>{c}</div>
                }
            })}
        </div>
    }
}
