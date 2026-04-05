use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkGroupIslandItem {
    pub label:    String,
    pub href:     String,
    pub active:   bool,
    pub disabled: bool,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum LinkGroupIslandDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[island]
pub fn LinkGroupIsland(
    items: Vec<LinkGroupIslandItem>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] direction: Option<LinkGroupIslandDirection>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class     = class.unwrap_or_default();
    let direction = direction.unwrap_or_default();
    let dir_str   = match direction {
        LinkGroupIslandDirection::Vertical   => "vertical",
        LinkGroupIslandDirection::Horizontal => "horizontal",
    };

    let items_view = items.iter().map(|item| {
        let label    = item.label.clone();
        let href     = item.href.clone();
        let active   = item.active;
        let disabled = item.disabled;
        let state    = if active { "active" } else if disabled { "disabled" } else { "inactive" };
        view! {
            <a
                data-rs-nav-item=""
                data-rs-state=state
                href=href
                aria-current=if active { "page" } else { "false" }
                aria-disabled=disabled.to_string()
            >
                <span data-rs-nav-item-label="">{label}</span>
            </a>
        }
    }).collect::<Vec<_>>();

    view! {
        <nav
            data-rs-navigation-group=""
            data-rs-component="LinkGroup"
            class=class
        >
            {label.map(|l| view! {
                <p data-rs-navigation-group-label="">{l}</p>
            })}
            <div data-rs-link-group-items="" data-rs-direction=dir_str>
                {items_view}
            </div>
        </nav>
    }
}
