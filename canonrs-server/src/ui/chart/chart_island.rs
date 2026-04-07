use leptos::prelude::*;
use super::chart_ui::Chart;
use canonrs_core::primitives::ChartType;
use canonrs_core::ChartData;

#[island]
pub fn ChartInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        let f = Closure::wrap(Box::new(move || {
            crate::interactions::chart::init_all();
        }) as Box<dyn Fn()>);
        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();
        f.forget();
    }
    view! { <></> }
}

#[component]
pub fn ChartIsland(
    data: ChartData,
    #[prop(optional, into)] chart_type: Option<String>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional)] show_grid: Option<bool>,
    #[prop(optional)] show_legend: Option<bool>,
    #[prop(optional)] animate: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let ct = match chart_type.as_deref() {
        Some("bar")   => ChartType::Bar,
        Some("area")  => ChartType::Area,
        Some("donut") => ChartType::Donut,
        _             => ChartType::Line,
    };
    view! {
        <ChartInit />
        <Chart
            data=data
            chart_type=ct
            height=height.unwrap_or(320)
            show_grid=show_grid.unwrap_or(true)
            show_legend=show_legend.unwrap_or(true)
            animate=animate.unwrap_or(true)
            class=class.unwrap_or_default()
            value=value.unwrap_or_default()
            aria_label=aria_label.unwrap_or_default()
        />
    }
}
