//! Chart UI - Enterprise Canvas chart component

use leptos::prelude::*;
use crate::primitives::ChartPrimitive;

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
            .map(|l| format!("\"{}\"", l))
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ChartType {
    #[default]
    Line,
    Bar,
    Area,
    Donut,
}

impl ChartType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChartType::Line => "line",
            ChartType::Bar => "bar",
            ChartType::Area => "area",
            ChartType::Donut => "donut",
        }
    }
}

#[component]
pub fn Chart(
    #[prop(into)] id: String,
    data: ChartData,
    #[prop(default = ChartType::Line)] chart_type: ChartType,
    #[prop(default = 320u32)] height: u32,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] show_tooltip: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] sync_table: String,
) -> impl IntoView {
    let json = data.to_json();

    view! {
        <ChartPrimitive
            id={id}
            class={class}
            chart_type={chart_type.as_str().to_string()}
            data={json}
            height={height}
            show_grid={show_grid}
            show_legend={show_legend}
            show_tooltip={show_tooltip}
            animate={animate}
            sync_table={sync_table}
        />
    }
}
