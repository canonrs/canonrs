use leptos::prelude::*;
use super::link_ui::Link;
use canonrs_core::primitives::LinkVariant;

#[component]
pub fn LinkIsland(
    children: Children,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] variant: Option<LinkVariant>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] external: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <Link
            href=href.unwrap_or_default()
            variant=variant.unwrap_or_default()
            disabled=disabled.unwrap_or(false)
            external=external.unwrap_or(false)
            class=class.unwrap_or_default()
        >
            {children()}
        </Link>
    }
}
