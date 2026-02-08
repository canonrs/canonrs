use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::toast::ToastClose;

#[component]
pub fn ToastCloseInteractive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <ToastClose 
            class=class 
            id=id
            on:click=move |_ev: ev::MouseEvent| {
                if let Some(ref handler) = on_click {
                    handler.run(());
                }
            }
        >
            {children.map(|c| c())}
        </ToastClose>
    }
}
