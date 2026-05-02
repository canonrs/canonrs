use leptos::prelude::*;
pub use canonrs_core::primitives::layout::SplitRatio;

#[component]
pub fn SplitViewLayout(
    #[prop(default = SplitRatio::Equal)] ratio: SplitRatio,
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let left  = StoredValue::new(left);
    let right = StoredValue::new(right);
    view! {
        <div data-rs-layout="split-view" data-rs-ratio=ratio.as_str() class=class>
            {move || left.get_value().map(|l| view! { <div data-rs-region="left" role="region" aria-label="Left panel">{l()}</div> })}
            {move || right.get_value().map(|r| view! { <div data-rs-region="right" role="region" aria-label="Right panel">{r()}</div> })}
        </div>
    }
}
