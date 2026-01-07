use leptos::prelude::*;

#[component]
pub fn BreadcrumbItem(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <li class=class>
            {children()}
        </li>
    }
}
