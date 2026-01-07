use leptos::prelude::*;

#[component]
pub fn BreadcrumbList(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <ol class=class>
            {children()}
        </ol>
    }
}
