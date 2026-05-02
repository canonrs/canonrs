//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - Canvas + overlay enterprise architecture

use leptos::prelude::*;
use crate::meta::{ChartGridState, ChartLegendState};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ChartType {
    #[default]
    Bar, Line, Area, Pie, Donut, Scatter, Radar,
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

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ChartAnimation {
    #[default]
    Auto,
    None,
}
impl ChartAnimation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
        }
    }
}

#[component]
pub fn ChartPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ChartType::Bar)] chart_type: ChartType,
    #[prop(into, default = "320".to_string())] height: String,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = ChartGridState::Visible)] chart_grid: ChartGridState,
    #[prop(default = ChartLegendState::Visible)] chart_legend: ChartLegendState,
    #[prop(default = ChartAnimation::Auto)] animation: ChartAnimation,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("ch");
    view! {
        <div
            data-rs-chart=""
            data-rs-uid=uid
            data-rs-interaction="data"
            data-rs-chart-type=chart_type.as_str()
            data-rs-chart-height=height
            data-rs-animation=animation.as_str()
            data-rs-value=value
            role="img"
            aria-label=aria_label
            class=class
        >
            <canvas data-rs-chart-canvas="" aria-hidden="true" />
            <div data-rs-chart-overlay="" aria-hidden="true">
                <div data-rs-chart-tooltip="" />
                <div data-rs-chart-crosshair="" />
            </div>
            <div data-rs-chart-legend="" data-rs-state=chart_legend.as_str() />
            <div data-rs-chart-grid="" data-rs-state=chart_grid.as_str() />
            {children()}
        </div>
    }
}

/// ChartDataPrimitive — injeta dados via data attribute
/// O runtime lê data-rs-chart-data para renderizar o gráfico
#[component]
pub fn ChartDataPrimitive(
    #[prop(into, default = String::new())] data: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-chart-data=data
            aria-hidden="true"
        />
    }
}
