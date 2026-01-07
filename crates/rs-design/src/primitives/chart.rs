//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn SvgPrimitive(
    #[prop(optional)] width: Option<String>,
    #[prop(optional)] height: Option<String>,
    #[prop(optional)] view_box: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <svg
            width=width.unwrap_or("100%".to_string())
            height=height.unwrap_or("100%".to_string())
            viewBox=view_box
        >
            {children()}
        </svg>
    }
}

#[component]
pub fn PathPrimitive(
    #[prop(optional)] d: Option<String>,
    #[prop(optional)] fill: Option<String>,
    #[prop(optional)] stroke: Option<String>,
    #[prop(optional)] stroke_width: Option<String>,
) -> impl IntoView {
    view! {
        <path
            d=d
            fill=fill
            stroke=stroke
            stroke-width=stroke_width
        />
    }
}

#[component]
pub fn RectPrimitive(
    #[prop(optional)] x: Option<f64>,
    #[prop(optional)] y: Option<f64>,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
    #[prop(optional)] fill: Option<String>,
) -> impl IntoView {
    view! {
        <rect
            x=x.unwrap_or(0.0)
            y=y.unwrap_or(0.0)
            width=width.unwrap_or(0.0)
            height=height.unwrap_or(0.0)
            fill=fill
        />
    }
}
