use leptos::prelude::*;

#[component]
pub fn ThreePaneLayout(
    #[prop(optional)] left:   Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right:  Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let left   = StoredValue::new(left);
    let center = StoredValue::new(center);
    let right  = StoredValue::new(right);
    view! {
        <div data-rs-layout="three-pane" class=class>
            {move || left.get_value().map(|l| view! { <nav data-rs-region="left" aria-label="Left panel">{l()}</nav> })}
            {move || center.get_value().map(|c| view! { <main data-rs-region="center">{c()}</main> })}
            {move || right.get_value().map(|r| view! { <aside data-rs-region="right" aria-label="Right panel">{r()}</aside> })}
        </div>
    }
}
