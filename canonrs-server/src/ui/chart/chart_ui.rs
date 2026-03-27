//! @canon-id: chart
//! @canon-label: Chart
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Visualize data graphically
//! @canon-description: Data chart visualization
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: chart, graph, bar, line, pie, area, data, visualization

use leptos::prelude::*;
use canonrs_core::primitives::{ChartPrimitive, ChartType};
pub use canonrs_core::primitives::ChartType as ChartKind;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartSeries {
    pub name: String,
    pub data: Vec<f64>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub series: Vec<ChartSeries>,
}

impl ChartData {
    pub fn to_json(&self) -> String {
        let labels = self.labels.iter()
            .map(|l| format!("{:?}", l))
            .collect::<Vec<_>>()
            .join(",");
        let series = self.series.iter().map(|s| {
            let data = s.data.iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let color = s.color.as_deref().unwrap_or("");
            format!(r#"{{"name":"{}","data":[{}],"color":"{}"}}"#, s.name, data, color)
        }).collect::<Vec<_>>().join(",");
        format!(r#"{{"labels":[{}],"series":[{}]}}"#, labels, series)
    }
}

#[component]
pub fn Chart(
    data: ChartData,
    #[prop(default = ChartType::Line)] chart_type: ChartType,
    #[prop(default = 320u32)] height: u32,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] show_tooltip: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] max_width: Option<u32>,
    #[prop(into, default = String::new())] sync_table: String,
    #[prop(into, default = String::new())] sync_scope: String,
) -> impl IntoView {
    let json = data.to_json();
    view! {
        <ChartPrimitive
            class=class
            chart_type=chart_type
            height=height
        >
            <script
                type="application/json"
                data-rs-chart-data=""
                data-rs-chart-grid=show_grid.to_string()
                data-rs-chart-legend=show_legend.to_string()
                data-rs-chart-tooltip=show_tooltip.to_string()
                data-rs-chart-animate=animate.to_string()
                data-rs-chart-sync-table=sync_table
                data-rs-chart-sync-scope=sync_scope
                data-rs-chart-max-width=max_width.unwrap_or(0).to_string()
                inner_html=json
            />
        </ChartPrimitive>
    }
}

#[component]
pub fn ChartPreview() -> impl IntoView {
    let data = ChartData {
        labels: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        series: vec![ChartSeries { name: "Series".to_string(), data: vec![10.0, 20.0, 15.0], color: None }],
    };
    view! { <Chart data=data height=120u32 /> }
}
