use leptos::prelude::*;
use crate::primitives::{
    ChartPrimitive, ChartContainerPrimitive, ChartContentPrimitive,
    ChartTooltipPrimitive, ChartTooltipContentPrimitive,
    ChartLegendPrimitive, ChartLegendItemPrimitive,
    ChartAxisPrimitive, ChartGridPrimitive
};

#[component]
pub fn Chart(
    children: Children,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ChartPrimitive 
            id={id.unwrap_or_default()} 
            class={class.unwrap_or_default()}
        >
            {children()}
        </ChartPrimitive>
    }
}

#[component]
pub fn ChartContainer(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartContainerPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartContainerPrimitive>
    }
}

#[component]
pub fn ChartContent(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartContentPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartContentPrimitive>
    }
}

#[component]
pub fn ChartTooltip(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartTooltipPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartTooltipPrimitive>
    }
}

#[component]
pub fn ChartTooltipContent(
    children: Children,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartTooltipContentPrimitive 
            label={label.unwrap_or_default()} 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartTooltipContentPrimitive>
    }
}

#[component]
pub fn ChartLegend(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartLegendPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartLegendPrimitive>
    }
}

#[component]
pub fn ChartLegendItem(
    children: Children,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartLegendItemPrimitive 
            color={color.unwrap_or_default()} 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </ChartLegendItemPrimitive>
    }
}

#[component]
pub fn ChartAxis(
    #[prop(default = "x")] axis: &'static str,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartAxisPrimitive
            axis={axis}
            label={label.unwrap_or_default()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        />
    }
}

#[component]
pub fn ChartGrid(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ChartGridPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()} 
        />
    }
}
