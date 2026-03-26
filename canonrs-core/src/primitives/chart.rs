//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - Canvas + overlay enterprise architecture

use leptos::prelude::*;

#[component]
pub fn ChartPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] chart_type: String,
    #[prop(default = 320u32)] height: u32,
    #[prop(optional, into)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-rs-chart=""
            data-rs-component="Chart"
            data-rs-behavior="data-display"
            data-rs-chart-type=chart_type
            data-rs-chart-height=height.to_string()
            role="img"
            aria-label=aria_label
            class=class
        >
            <canvas data-rs-chart-canvas="" aria-hidden="true" />
            <div data-rs-chart-overlay="" aria-hidden="true">
                <div data-rs-chart-tooltip="" data-rs-state="closed" />
                <div data-rs-chart-crosshair="" data-rs-state="closed" />
            </div>
            <div data-rs-chart-legend="" />
            {children()}
        </div>
    }
}
