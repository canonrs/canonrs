use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TabItem {
    pub value:    String,
    pub label:    String,
    pub content:  String,
    pub disabled: bool,
}

#[island]
pub fn TabsIsland(
    tabs: Vec<TabItem>,
    #[prop(into)] active: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let (active_tab, set_active_tab) = signal(active.clone());

    let initial_state_trigger = |value: &str| -> String {
        if value == active { "active".into() } else { "inactive".into() }
    };

    let initial_state_content = |value: &str| -> String {
        if value == active { "active".into() } else { "inactive".into() }
    };

    let triggers_view = tabs.iter().map(|tab| {
        let value     = tab.value.clone();
        let label     = tab.label.clone();
        let disabled  = tab.disabled;
        let initial   = initial_state_trigger(&value);

        let on_click = {
            let value = value.clone();
            move |_: leptos::ev::MouseEvent| {
                if !disabled { set_active_tab.set(value.clone()); }
            }
        };

        #[cfg(feature = "hydrate")]
        let on_keydown = {
            let value = value.clone();
            move |e: leptos::ev::KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" | " " => {
                        e.prevent_default();
                        if !disabled { set_active_tab.set(value.clone()); }
                    }
                    _ => {}
                }
            }
        };
        #[cfg(not(feature = "hydrate"))]
        let on_keydown = move |_: leptos::ev::KeyboardEvent| {};

        let item_state = {
            let value = value.clone();
            move || if active_tab.get() == value { "active".to_string() } else { "inactive".to_string() }
        };

        view! {
            <button
                type="button"
                role="tab"
                data-rs-tabs-trigger=""
                data-rs-value=value.clone()
                data-rs-state=move || { let s = item_state(); if s.is_empty() { initial.clone() } else { s } }
                aria-selected=move || (active_tab.get() == value).to_string()
                aria-disabled=disabled.to_string()
                disabled=disabled
                on:click=on_click
                on:keydown=on_keydown
            >
                {label}
            </button>
        }
    }).collect::<Vec<_>>();

    let contents_view = tabs.iter().map(|tab| {
        let value    = tab.value.clone();
        let content  = tab.content.clone();
        let initial  = initial_state_content(&value);

        let item_state = {
            let value = value.clone();
            move || if active_tab.get() == value { "active".to_string() } else { "inactive".to_string() }
        };

        let is_hidden = {
            let value = value.clone();
            move || active_tab.get() != value
        };

        view! {
            <div
                data-rs-tabs-content=""
                data-rs-value=value.clone()
                data-rs-state=move || { let s = item_state(); if s.is_empty() { initial.clone() } else { s } }
                role="tabpanel"
                hidden=is_hidden
            >
                {content}
            </div>
        }
    }).collect::<Vec<_>>();

    view! {
        <div
            data-rs-tabs=""
            data-rs-component="Tabs"
            class=class
        >
            <div
                data-rs-tabs-list=""
                role="tablist"
                aria-orientation="horizontal"
            >
                {triggers_view}
            </div>
            {contents_view}
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabsContext(pub RwSignal<String>);

#[island]
pub fn TabsRootIsland(
    children: Children,
    #[prop(into)] initial: String,
) -> impl IntoView {
    let active = RwSignal::new(initial);
    provide_context(TabsContext(active));
    view! {
        <div data-rs-tabs="" data-rs-component="Tabs">
            {children()}
        </div>
    }
}

#[island]
pub fn TabsTriggerIsland(
    children: Children,
    #[prop(into)] value: String,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    use canonrs_core::meta::ActivityState;
    let disabled = disabled.unwrap_or(false);
    let ctx = use_context::<TabsContext>();
    let value2 = value.clone();

    let signal = ctx.as_ref().map(|TabsContext(a)| *a);
    let value3 = value2.clone();
    let state = move || {
        signal.map(|a| if a.get() == value2 { ActivityState::Active.as_str() } else { ActivityState::Inactive.as_str() })
              .unwrap_or(ActivityState::Inactive.as_str())
    };
    let is_active = move || signal.map(|a| a.get() == value3).unwrap_or(false);

    #[cfg(feature = "hydrate")]
    let on_click = {
        let value = value.clone();
        let ctx = ctx.clone();
        move |_: leptos::ev::MouseEvent| {
            if !disabled {
                if let Some(TabsContext(active)) = ctx.as_ref() {
                    active.set(value.clone());
                }
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| {};

    view! {
        <button
            type="button"
            role="tab"
            data-rs-tabs-trigger=""
            data-rs-value=value.clone()
            data-rs-state=state
            aria-selected=is_active
            disabled=disabled
            on:click=on_click
        >
            {children()}
        </button>
    }
}

#[island]
pub fn TabsContentIsland(
    children: Children,
    #[prop(into)] value: String,
) -> impl IntoView {
    let ctx = use_context::<TabsContext>();
    let value2 = value.clone();

    use canonrs_core::meta::ActivityState;
    let signal = ctx.as_ref().map(|TabsContext(a)| *a);
    let value3 = value2.clone();
    let state = move || {
        signal.map(|a| if a.get() == value2 { ActivityState::Active.as_str() } else { ActivityState::Inactive.as_str() })
              .unwrap_or(ActivityState::Inactive.as_str())
    };
    let hidden = move || signal.map(|a| a.get() != value3).unwrap_or(true);

    view! {
        <div
            data-rs-tabs-content=""
            data-rs-value=value
            data-rs-state=state
            hidden=hidden
            role="tabpanel"
        >
            {children()}
        </div>
    }
}
