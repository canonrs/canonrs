use leptos::prelude::*;
use crate::primitives::{StatPrimitive, StatValuePrimitive, StatLabelPrimitive};

#[component]
pub fn Stat(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <StatPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </StatPrimitive>
    }
}

#[component]
pub fn StatValue(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <StatValuePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </StatValuePrimitive>
    }
}

#[component]
pub fn StatLabel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <StatLabelPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </StatLabelPrimitive>
    }
}
