use leptos::prelude::*;

#[component]
pub fn KbdPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <kbd
            id={id}
            class={class}
            data-kbd=""
        >
            {children.map(|c| c())}
        </kbd>
    }
}
