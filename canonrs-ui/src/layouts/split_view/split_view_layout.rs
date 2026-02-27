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
    #[prop(default = Signal::derive(|| String::new()))] left_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] right_zone_id: Signal<String>,
    left: Children,
    right: Children,
) -> impl IntoView {
    view! {
        <div class="layout-split-view" data-layout="split-view" data-layout-version="1"
            attr:data-split-ratio=ratio.as_str()>
            <div class="layout-split-left"
                data-layout-region="left"
                data-region-hint="Drop context or branding"
                data-drop-zone=move || (!left_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!left_zone_id.get().is_empty()).then(|| left_zone_id.get())>
                {left()}
            </div>
            <div class="layout-split-right"
                data-layout-region="right"
                data-region-hint="Drop action or form"
                data-drop-zone=move || (!right_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!right_zone_id.get().is_empty()).then(|| right_zone_id.get())>
                {right()}
            </div>
        </div>
    }
}
