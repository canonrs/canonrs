use crate::primitives::collapsible::*;
use leptos::prelude::*;

#[component]
pub fn Collapsible(
    children: ChildrenFn,
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <CollapsiblePrimitive open=open on_open_change=on_open_change>
            {children()}
        </CollapsiblePrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <CollapsibleTriggerPrimitive class=class>
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsibleContent(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <CollapsibleContentPrimitive class=class>
            {children()}
        </CollapsibleContentPrimitive>
    }
}
