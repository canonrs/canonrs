use leptos::prelude::*;
use crate::primitives::button_group::ButtonGroupPrimitive;

#[component]
pub fn ButtonGroup(
    #[prop(optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] attached: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ButtonGroupPrimitive
            id={id.unwrap_or_default()}
            class={class}
            attached={attached}
            aria_label={aria_label.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </ButtonGroupPrimitive>
    }
}
