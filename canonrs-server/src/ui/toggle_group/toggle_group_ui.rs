use leptos::prelude::*;
use super::toggle_group_primitive::ToggleGroupPrimitive;

#[component]
pub fn ToggleGroup(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] multiple: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("toggle-group {}", class);

    view! {
        <ToggleGroupPrimitive
            id={id}
            class={base_class}
            role="group".to_string()
            multiple={multiple}
        >
            {children.map(|c| c())}
        </ToggleGroupPrimitive>
    }
}
