//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - Canvas + overlay enterprise architecture

use leptos::prelude::*;
use crate::meta::{ActivityState, ChartGridState, ChartLegendState};
use crate::infra::state_engine::activity_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
    Area,
    Pie,
    Donut,
    Scatter,
    Radar,
}
impl ChartType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bar     => "bar",
            Self::Line    => "line",
            Self::Area    => "area",
            Self::Pie     => "pie",
            Self::Donut   => "donut",
            Self::Scatter => "scatter",
            Self::Radar   => "radar",
        }
    }
}

#[component]
pub fn ChartPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ChartType::Bar)] chart_type: ChartType,
    #[prop(default = 320u32)] height: u32,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, default = String::new())] chart_data: String,
    #[prop(default = ChartGridState::Visible)] chart_grid: ChartGridState,
    #[prop(default = ChartLegendState::Visible)] chart_legend: ChartLegendState,
    #[prop(default = true)] chart_animate: bool,
    children: Children,
) -> impl IntoView {
    let aria = aria_label.unwrap_or_else(|| format!("{} chart", chart_type.as_str()));
    let a = activity_attrs(if chart_animate { ActivityState::Active } else { ActivityState::Inactive });
    view! {
        <div
            data-rs-chart=""
            data-rs-component="Chart"
            data-rs-behavior="chart"
            data-rs-chart-type=chart_type.as_str()
            data-rs-chart-height={height.to_string()}
            data-rs-state=a.data_rs_state
            data-rs-value=value.unwrap_or_default()
            role="img"
            aria-label=aria
            class=class
        >
            <canvas data-rs-chart-canvas="" aria-hidden="true" />
            <div data-rs-chart-overlay="" aria-hidden="true">
                <div data-rs-chart-tooltip="" data-rs-state="inactive" />
                <div data-rs-chart-crosshair="" data-rs-state="inactive" />
            </div>
            <div data-rs-chart-legend="" data-rs-state=chart_legend.as_str() />
            <div data-rs-chart-grid="" data-rs-state=chart_grid.as_str() />
            <script data-rs-chart-data="" type="application/json">{chart_data}</script>
            {children()}
        </div>
    }
}
