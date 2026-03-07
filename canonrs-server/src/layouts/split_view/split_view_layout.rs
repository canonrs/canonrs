//! # SplitViewLayout — Regions: left, right
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SplitRatio { Equal, FormFocused, ContextFocused }

impl SplitRatio {
    fn as_str(&self) -> &'static str {
        match self {
            SplitRatio::Equal => "50-50",
            SplitRatio::FormFocused => "40-60",
            SplitRatio::ContextFocused => "60-40",
        }
    }
}

#[component]
pub fn SplitViewLayout(
    #[prop(default = SplitRatio::Equal)] ratio: SplitRatio,
    left: Children,
    right: Children,
) -> impl IntoView {
    view! {
        <div class="layout-split-view" data-layout="split-view" data-layout-version="1"
            attr:data-split-ratio=ratio.as_str()>
            <div class="layout-split-left" data-layout-region="left">{left()}</div>
            <div class="layout-split-right" data-layout-region="right">{right()}</div>
        </div>
    }
}
