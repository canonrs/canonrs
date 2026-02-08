use leptos::prelude::*;
use super::input_group_primitive::InputGroupPrimitive;

#[component]
pub fn InputGroup(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("input-group {}", class);

    view! {
        <InputGroupPrimitive
            id={id}
            class={base_class}
        >
            {children.map(|c| c())}
        </InputGroupPrimitive>
    }
}
