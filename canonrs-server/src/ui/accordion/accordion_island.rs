use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccordionIslandItem {
    pub value:    String,
    pub trigger:  String,
    pub content:  String,
    pub disabled: bool,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AccordionSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[island]
pub fn AccordionIsland(
    items: Vec<AccordionIslandItem>,
    #[prop(optional)] selection: Option<AccordionSelectionMode>,
    #[prop(optional)] collapsible: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let selection    = selection.unwrap_or_default();
    let collapsible  = collapsible.unwrap_or(true);
    let class        = class.unwrap_or_default();
    let is_single    = selection == AccordionSelectionMode::Single;

    // RwSignal<Vec<String>> — lista de valores abertos
    let open_items   = RwSignal::new(Vec::<String>::new());

    let items_view = items.iter().map(|item| {
        let value    = item.value.clone();
        let trigger  = item.trigger.clone();
        let content  = item.content.clone();
        let disabled = item.disabled;
        let v2       = value.clone();


        let v2c = v2.clone();
        let v2d = v2.clone();
        let v2e = v2.clone();
        let initial = "closed";

        let state        = move || if open_items.get().contains(&v2)  { "open" } else { "closed" };
        let is_expanded  = move || open_items.get().contains(&v2c);
        let is_hidden    = move || !open_items.get().contains(&v2d);
        let aria_hidden  = move || (!open_items.get().contains(&v2e)).to_string();

        #[cfg(feature = "hydrate")]
        let on_click = {
            let value = value.clone();
            move |_: leptos::ev::MouseEvent| {
                if disabled { return; }
                open_items.update(|items| {
                    if items.contains(&value) {
                        if collapsible { items.retain(|v| v != &value); }
                    } else {
                        if is_single { items.clear(); }
                        items.push(value.clone());
                    }
                });
            }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_click = move |_: leptos::ev::MouseEvent| {};

        #[cfg(feature = "hydrate")]
        let on_keydown = {
            let value = value.clone();
            move |e: leptos::ev::KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" | " " => {
                        e.prevent_default();
                        if disabled { return; }
                        open_items.update(|items| {
                            if items.contains(&value) {
                                if collapsible { items.retain(|v| v != &value); }
                            } else {
                                if is_single { items.clear(); }
                                items.push(value.clone());
                            }
                        });
                    }
                    _ => {}
                }
            }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_keydown = move |_: leptos::ev::KeyboardEvent| {};

        view! {
            <div
                data-rs-accordion-item=""
                data-rs-state=move || { let s = state(); if s.is_empty() { initial } else { s } }
            >
                <button
                    type="button"
                    data-rs-accordion-trigger=""
                    aria-expanded=move || is_expanded().to_string()
                    disabled=disabled
                    on:click=on_click
                    on:keydown=on_keydown
                >
                    {trigger}
                </button>
                <div
                    data-rs-accordion-content=""
                    hidden=is_hidden
                    aria-hidden=aria_hidden
                >
                    {content}
                </div>
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-accordion=""
            data-rs-component="Accordion"
            data-rs-selection=if is_single { "single" } else { "multiple" }
            data-rs-collapsible=collapsible.to_string()
            class=class
        >
            {items_view}
        </div>
    }
}
