use leptos::prelude::*;
use super::button_group_primitive::ButtonGroupPrimitive;

#[component]
pub fn ButtonGroup(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("button-group {}", class);

    view! {
        <ButtonGroupPrimitive
            id={id}
            class={base_class}
            role="group".to_string()
        >
            {children.map(|c| c())}
        </ButtonGroupPrimitive>
    }
}
