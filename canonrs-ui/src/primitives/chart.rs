//! @canon-level: strict
//! @canon-owner: primitives-team
//! Chart Primitive - HTML puro para gráficos

use leptos::prelude::*;

#[component]
pub fn ChartPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    // ID deve ser fornecido pelo consumer
    
    view! {
        <div
            attr:data-chart={id}
            attr:data-slot="chart"
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartContainerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-chart-container="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-chart-content="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartTooltipPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-chart-tooltip=""
            role="tooltip"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartTooltipContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-chart-tooltip-content="" class={class} id={id}>
            {if !label.is_empty() { Some(view! {
                <div data-chart-tooltip-label="">
                    {label}
                </div>
            }) } else { None }}
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartLegendPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-chart-legend=""
            role="list"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartLegendItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] color: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-chart-legend-item=""
            role="listitem"
            class={class}
            id={id}
        >
            {if !color.is_empty() { Some(view! {
                <div
                    attr:data-chart-legend-indicator=""
                    style={format!("background-color: {};", color)}
                />
            }) } else { None }}
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ChartAxisPrimitive(
    #[prop(default = "x")] axis: &'static str,
    #[prop(default = String::new())] label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-chart-axis=""
            attr:data-axis={axis}
            attr:aria-label={label}
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn ChartGridPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-chart-grid=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        />
    }
}
