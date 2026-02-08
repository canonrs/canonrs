use leptos::callback::Callback;
//! Interactive Button - wrapper com callback support
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::primitives::ButtonPrimitive;

#[component]
pub fn ButtonInteractive(
    #[prop(optional)] children: Option<Children>,
    on_click: Callback<ev::MouseEvent>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class=class
            id=id
            disabled=disabled
            on:click=move |ev| on_click.run(ev)
        >
            {children.map(|c| c())}
        </ButtonPrimitive>
    }
}
