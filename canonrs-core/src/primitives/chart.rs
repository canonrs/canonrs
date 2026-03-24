//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - Canvas + overlay enterprise architecture

use leptos::prelude::*;

#[component]
pub fn ChartPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] chart_type: String,
    #[prop(default = 320u32)] height: u32,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id=id
            class=class
            data-rs-chart=""
            data-rs-chart-type=chart_type
            data-rs-chart-height=height.to_string()
        >
            <canvas data-rs-chart-canvas="" />

            <div data-rs-chart-overlay="">
                <div
                    data-rs-chart-tooltip=""
                    data-rs-state="closed"
                />
                <div
                    data-rs-chart-crosshair=""
                    data-rs-state="closed"
                />
            </div>

            <div data-rs-chart-legend="" />

            {children.map(|c| c())}
        </div>
    }
}
