use leptos::prelude::*;
use crate::primitives::PulsePrimitive;

#[component]
pub fn Pulse(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PulsePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </PulsePrimitive>
    }
}
