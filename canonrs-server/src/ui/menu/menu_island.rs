use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuIslandItem {
    pub label:    String,
    pub value:    String,
    pub disabled: bool,
}

#[island]
pub fn MenuIsland(
    items: Vec<MenuIslandItem>,
    #[prop(optional, into)] selected: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class      = class.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let (active, set_active) = signal(selected.unwrap_or_default());
    let _ = set_active;

    let items_view = items.iter().map(|item| {
        let value    = item.value.clone();
        let label    = item.label.clone();
        let disabled = item.disabled;
        let v2       = value.clone();
        let _v3      = value.clone();

        let is_selected  = move || active.get() == value;
        let is_selected2 = move || active.get() == v2;
        let state        = move || if is_selected() { "selected" } else { "unselected" };

        #[cfg(feature = "hydrate")]
        let on_click = move |_: leptos::ev::MouseEvent| {
            if !disabled { set_active.set(_v3.clone()); }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_click = move |_: leptos::ev::MouseEvent| {};

        view! {
            <div
                data-rs-menu-item=""
                role="menuitem"
                data-rs-state=state
                aria-selected=move || is_selected2().to_string()
                aria-disabled=disabled.to_string()
                on:click=on_click
            >
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-menu=""
            data-rs-component="Menu"
            role="menu"
            aria-label=aria_label
            class=class
        >
            {items_view}
        </div>
    }
}
