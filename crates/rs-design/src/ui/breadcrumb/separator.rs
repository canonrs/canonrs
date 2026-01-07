use leptos::prelude::*;

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <li role="presentation" aria-hidden="true" class=class>
            {if let Some(children) = children {
                children().into_any()
            } else {
                view! { <span>"/"</span> }.into_any()
            }}
        </li>
    }
}
