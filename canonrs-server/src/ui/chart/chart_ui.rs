#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{ChartPrimitive, ChartDataPrimitive, ChartType, ChartAnimation};
use canonrs_core::{ChartGridState, ChartLegendState};
pub use canonrs_core::primitives::ChartType as ChartKind;
pub use canonrs_core::{ChartData, ChartSeries};

#[component]
pub fn Chart(
    data: ChartData,
    #[prop(default = ChartType::Line)] chart_type: ChartType,
    #[prop(into, default = "320".to_string())] height: String,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] _show_tooltip: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] _max_width: Option<u32>,
    #[prop(into, default = String::new())] _sync_table: String,
    #[prop(into, default = String::new())] _sync_scope: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let json = data.to_json();
    let animation = if animate { ChartAnimation::Auto } else { ChartAnimation::None };
    view! {
        <ChartPrimitive
            class=class
            chart_type=chart_type
            height=height
            value=value
            aria_label=aria_label.unwrap_or_default()
            chart_grid=ChartGridState::from(show_grid)
            chart_legend=ChartLegendState::from(show_legend)
            animation=animation
        >
            <ChartDataPrimitive data=json />
        </ChartPrimitive>
    }
}

#[component]
pub fn ChartPreview() -> impl IntoView {
    let data = ChartData {
        labels: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        series: vec![ChartSeries { name: "Series".to_string(), data: vec![10.0, 20.0, 15.0], color: None }],
    };
    view! { <Chart data=data height="120".to_string() /> }
}
