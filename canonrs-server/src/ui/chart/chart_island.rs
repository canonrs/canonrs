use leptos::prelude::*;
use super::chart_ui::Chart;
use canonrs_core::primitives::ChartType;
use canonrs_core::ChartData;

#[island]
pub fn ChartInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn ChartIsland(
    data: ChartData,
    #[prop(default = ChartType::Line)] chart_type: ChartType,
    #[prop(default = 320u32)] height: u32,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <ChartInit />
        <Chart
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
