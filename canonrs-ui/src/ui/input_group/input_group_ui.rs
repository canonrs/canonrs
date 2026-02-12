use leptos::prelude::*;
use crate::primitives::InputGroupPrimitive;

#[component]
pub fn InputGroup(
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] merge_radius: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <InputGroupPrimitive
            id={id.unwrap_or_default()}
            class={class}
            merge_radius={merge_radius}
        >
            {children.map(|c| c())}
        </InputGroupPrimitive>
    }
}
