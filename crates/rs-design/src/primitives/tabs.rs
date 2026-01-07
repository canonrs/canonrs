//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn TabsPrimitive(children: Children, #[prop(into)] value: RwSignal<String>) -> impl IntoView {
    provide_context(value);

    view! {
        <div data-name="Tabs">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-name="TabsList" role="tablist">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    children: Children, 
    #[prop(into)] value: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let active_value = expect_context::<RwSignal<String>>();
    let value_for_state = value.clone();
    let value_for_aria = value.clone();
    let value_for_click = value.clone();

    view! {
        <button
            class=class
            data-name="TabsTrigger"
            data-state=move || if active_value.get() == value_for_state { "active" } else { "inactive" }
            role="tab"
            aria-selected=move || active_value.get() == value_for_aria
            on:click=move |_| active_value.set(value_for_click.clone())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContentPrimitive(children: Children, #[prop(into)] value: String) -> impl IntoView {
    let active_value = expect_context::<RwSignal<String>>();
    let value_for_state = value.clone();
    let value_for_style = value.clone();

    view! {
        <div
            data-name="TabsContent"
            data-state=move || if active_value.get() == value_for_state { "active" } else { "inactive" }
            role="tabpanel"
            style=move || if active_value.get() == value_for_style { "" } else { "display: none;" }
        >
            {children()}
        </div>
    }
}
