use leptos::prelude::*;

/// BreadcrumbNav - Semantic nav wrapper
/// 
/// **Type:** Pure Component (Type 1)
/// **HTML:** `<nav>`
/// **A11y:** aria-label="breadcrumb"
#[component]
pub fn BreadcrumbNav(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <nav aria-label="breadcrumb" class=class>
            {children()}
        </nav>
    }
}
