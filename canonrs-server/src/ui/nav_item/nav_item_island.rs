use leptos::prelude::*;

#[island]
pub fn NavItemIsland(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] active: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let href     = href.unwrap_or_default();
    let active   = active.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let class    = class.unwrap_or_default();
    let state    = if active { "active" } else if disabled { "disabled" } else { "inactive" };

    view! {
        <a
            data-rs-nav-item=""
            data-rs-component="NavItem"
            data-rs-state=state
            href=href
            aria-current=if active { "page" } else { "false" }
            aria-disabled=disabled.to_string()
            class=class
        >
            <span data-rs-nav-item-label="">{label}</span>
        </a>
    }
}
