//! Chart Behavior - orquestração

pub(crate) mod data;
pub(crate) mod draw;
pub(crate) mod legend;
pub(crate) mod resize;
pub(crate) mod sync;
pub(crate) mod tooltip;
pub(crate) mod utils;

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use data::{read_chart_data, parse_chart_data};
#[cfg(feature = "hydrate")]
use draw::{draw_chart, set_canvas_dpi};
#[cfg(feature = "hydrate")]
use legend::draw_legend;
#[cfg(feature = "hydrate")]
use resize::setup_resize_observer;
#[cfg(feature = "hydrate")]
use sync::setup_datatable_to_chart_sync;
#[cfg(feature = "hydrate")]
use tooltip::setup_tooltip;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub(crate) fn register() {
    register_behavior("data-rs-chart", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        let chart_type   = root.get_attribute("data-rs-chart-type").unwrap_or_else(|| "line".to_string());
        let height: f64  = root.get_attribute("data-rs-chart-height").and_then(|h| h.parse().ok()).unwrap_or(320.0);
        let show_grid = root.query_selector("[data-rs-chart-grid]").ok().flatten()
            .and_then(|el| el.get_attribute("data-rs-state"))
            .as_deref() != Some("hidden");
        let show_legend = root.query_selector("[data-rs-chart-legend]").ok().flatten()
            .and_then(|el| el.get_attribute("data-rs-state"))
            .as_deref() != Some("hidden");
        let show_tooltip = root.get_attribute("data-rs-chart-tooltip").as_deref() != Some("false");
        let sync_table   = root.get_attribute("data-rs-chart-sync-table");

        let data_json = read_chart_data(root);

        let canvas_el = match root.query_selector("[data-rs-chart-canvas]").ok().flatten() {
            Some(el) => el,
            None => return Ok(()),
        };
        let canvas: web_sys::HtmlCanvasElement = canvas_el.dyn_into()
            .map_err(|_| crate::BehaviorError::JsError { message: "canvas cast".into() })?;

        let (labels, series) = match parse_chart_data(&data_json) {
            Some(d) => d,
            None => return Ok(()),
        };

        {
            let canvas_raf   = canvas.clone();
            let root_raf     = root.clone();
            let labels_raf   = labels.clone();
            let series_raf   = series.clone();
            let chart_type_r = chart_type.clone();
            let cb = Closure::once(move || {
                set_canvas_dpi(&canvas_raf, &root_raf, height);
                draw_chart(&canvas_raf, &chart_type_r, &labels_raf, &series_raf, show_grid, height);
            });
            web_sys::window().unwrap().request_animation_frame(cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if show_legend {
            if let Some(legend_el) = root.query_selector("[data-rs-chart-legend]").ok().flatten() {
                draw_legend(&legend_el, &series, root, &canvas, &chart_type, &labels, show_grid, height);
            }
        }

        if show_tooltip {
            setup_tooltip(&canvas, root, &labels, &series, height, sync_table.clone());
        }

        setup_resize_observer(&canvas, root, &labels, &series, &chart_type, show_grid, height);

        if let Some(table_id) = sync_table {
            setup_datatable_to_chart_sync(root, &canvas, &table_id, &labels, &series, &chart_type, show_grid, height);
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub(crate) fn register() {}
