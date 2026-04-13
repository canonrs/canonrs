use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SplitRatio { #[default] Equal, FormFocused, ContextFocused }
impl SplitRatio {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equal          => "50-50",
            Self::FormFocused    => "40-60",
            Self::ContextFocused => "60-40",
        }
    }
}

#[component]
pub fn SplitViewLayout(
    #[prop(default = SplitRatio::Equal)] ratio: SplitRatio,
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid   = generate("ly");
    let left  = StoredValue::new(left);
    let right = StoredValue::new(right);
    view! {
        <div data-rs-layout-split-view="" data-rs-uid=uid data-rs-ratio=ratio.as_str() class=class>
            {move || left.get_value().map(|l| view! { <div data-rs-region="left">{l()}</div> })}
            {move || right.get_value().map(|r| view! { <div data-rs-region="right">{r()}</div> })}
        </div>
    }
}
