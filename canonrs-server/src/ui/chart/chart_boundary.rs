use leptos::prelude::*;
use super::chart_ui::Chart as ChartUi;
pub use canonrs_core::primitives::ChartType;
use canonrs_core::ChartData;

#[component]
pub fn Chart(
    data: ChartData,
    #[prop(default = ChartType::Line)] chart_type: ChartType,
    #[prop(into, default = "320".to_string())] height: String,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <ChartUi
            data=data
            chart_type=chart_type
            height=height
            show_grid=show_grid
            show_legend=show_legend
            animate=animate
            class=class
            value=value
            aria_label=aria_label
        />
    }
}
