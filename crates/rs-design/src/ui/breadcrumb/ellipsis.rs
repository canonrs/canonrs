use leptos::prelude::*;

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <span role="presentation" aria-hidden="true" class=class>
            "…"
            <span class="sr-only">"More"</span>
        </span>
    }
}
