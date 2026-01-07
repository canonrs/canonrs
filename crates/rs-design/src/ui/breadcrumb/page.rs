use leptos::prelude::*;

#[component]
pub fn BreadcrumbPage(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <span
            role="link"
            aria-disabled="true"
            aria-current="page"
            class=class
        >
            {children()}
        </span>
    }
}
