//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! DashboardStats Block - Grid de métricas

use crate::blocks::dashboard::StatCard;
use leptos::prelude::*;

#[component]
pub fn DashboardStats(#[prop(optional)] loading: bool) -> impl IntoView {
    // Simulando dados - em produção viriam de API
    let stats = vec![
        (
            "Total Revenue",
            "$1,250.00",
            "from last month",
            Some((12.5, true)),
        ),
        (
            "New Customers",
            "1,234",
            "from last month",
            Some((8.2, true)),
        ),
        ("Sales", "573", "from last month", Some((19.0, true))),
        ("Active Now", "89", "since last hour", Some((5.3, false))),
    ];

    view! {
        <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            {stats.into_iter().map(|(title, value, desc, trend)| {
                let trend_data = trend.map(|(pct, pos)| crate::blocks::dashboard::StatTrend {
                    percentage: pct,
                    positive: pos,
                });

                view! {
                    <StatCard
                        title=title.to_string()
                        value=value.to_string()
                        description=desc.to_string()
                        loading=loading
                        trend=trend_data
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
