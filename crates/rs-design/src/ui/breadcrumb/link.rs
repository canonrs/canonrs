use leptos::prelude::*;

#[component]
pub fn BreadcrumbLink(
    #[prop(into)] href: String,
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let handle_click = move |ev: leptos::ev::MouseEvent| {
        if let Some(handler) = on_click {
            ev.prevent_default();
            handler.run(());
        }
    };
    
    view! {
        <a href=href class=class on:click=handle_click>
            {children()}
        </a>
    }
}
