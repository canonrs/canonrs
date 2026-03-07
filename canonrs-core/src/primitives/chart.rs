//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - Canvas + overlay enterprise architecture

use leptos::prelude::*;

#[component]
pub fn ChartPrimitive(
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] chart_type: String,
    #[prop(into)] data: String,
    #[prop(default = 320u32)] height: u32,
    #[prop(default = true)] show_grid: bool,
    #[prop(default = true)] show_legend: bool,
    #[prop(default = true)] show_tooltip: bool,
    #[prop(default = true)] animate: bool,
    #[prop(into, default = String::new())] sync_table: String,
    #[prop(into, default = String::new())] sync_scope: String,
) -> impl IntoView {
    let canvas_id  = format!("{}-canvas", id);
    let tooltip_id = format!("{}-tooltip", id);

    view! {
        <div
            id={id}
            class={class}
            data-chart=""
            data-chart-type={chart_type}
            data-chart-height={height.to_string()}
            data-chart-grid={show_grid.to_string()}
            data-chart-legend={show_legend.to_string()}
            data-chart-tooltip={show_tooltip.to_string()}
            data-chart-animate={animate.to_string()}
            data-chart-sync-table={sync_table}
            data-sync-scope={sync_scope}
        >
            <canvas id={canvas_id} data-chart-canvas="" />

            <div data-chart-overlay="">
                <div id={tooltip_id} data-chart-tooltip-el="" data-state="hidden" />
                <div data-chart-crosshair="" data-state="hidden" />
            </div>

            <div data-chart-legend-el="" />

            <script
                type="application/json"
                data-chart-data=""
                inner_html={data}
            />
        </div>
    }
}
