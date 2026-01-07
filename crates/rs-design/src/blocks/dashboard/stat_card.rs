//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! StatCard Block - Métricas do dashboard

use crate::tokens::animations::AnimationVariant;
use crate::ui::{Animate, Card};
use leptos::prelude::*;

#[component]
pub fn StatCard(
    title: String,
    value: String,
    description: String,
    #[prop(optional)] loading: bool,
    trend: Option<StatTrend>,
) -> impl IntoView {
    view! {
        <Animate variant=AnimationVariant::FadeUp>
            <Card class="hover:shadow-lg transition-shadow">
                <div class="p-6">
                    {if loading {
                        view! {
                            <div class="animate-pulse">
                                <div class="h-4 bg-gray-200 rounded w-24 mb-2"></div>
                                <div class="h-8 bg-gray-300 rounded w-32 mb-2"></div>
                                <div class="h-3 bg-gray-200 rounded w-28"></div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <p class="text-sm text-gray-600 dark:text-gray-400">{title}</p>
                            <h3 class="text-2xl font-semibold mt-2">{value}</h3>
                            <div class="flex items-center gap-1 mt-2">
                                {trend.map(|t| view! {
                                    <span class=format!(
                                        "text-xs font-medium {}",
                                        if t.positive { "text-green-600" } else { "text-red-600" }
                                    )>
                                        {if t.positive { "↑" } else { "↓" }}
                                        {t.percentage}"%"
                                    </span>
                                })}
                                <span class="text-xs text-gray-500">{description}</span>
                            </div>
                        }.into_any()
                    }}
                </div>
            </Card>
        </Animate>
    }
}

#[derive(Clone)]
pub struct StatTrend {
    pub percentage: f32,
    pub positive: bool,
}
