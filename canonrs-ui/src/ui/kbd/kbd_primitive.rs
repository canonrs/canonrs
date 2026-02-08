use leptos::prelude::*;

#[component]
pub fn KbdPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <kbd
            id={id}
            class={class}
            attr:data-kbd=""
        >
            {children.map(|c| c())}
        </kbd>
    }
}
