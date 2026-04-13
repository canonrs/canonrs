use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[component]
pub fn ThreePaneLayout(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid    = generate("ly");
    let left   = StoredValue::new(left);
    let center = StoredValue::new(center);
    let right  = StoredValue::new(right);
    view! {
        <div data-rs-layout-three-pane="" data-rs-uid=uid class=class>
            {move || left.get_value().map(|l| view! { <div data-rs-region="left">{l()}</div> })}
            {move || center.get_value().map(|c| view! { <div data-rs-region="center">{c()}</div> })}
            {move || right.get_value().map(|r| view! { <div data-rs-region="right">{r()}</div> })}
        </div>
    }
}
