use leptos::prelude::*;
use super::kbd_primitive::KbdPrimitive;

#[component]
pub fn Kbd(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("kbd {}", class);

    view! {
        <KbdPrimitive
            id={id}
            class={base_class}
        >
            {children.map(|c| c())}
        </KbdPrimitive>
    }
}
