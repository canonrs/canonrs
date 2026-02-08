use leptos::prelude::*;
use crate::shared::DrawerVariant;
use crate::primitives::{DrawerPrimitive, DrawerTriggerPrimitive, DrawerContentPrimitive, DrawerOverlayPrimitive};

#[component]
pub fn Drawer(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    collapsed: Signal<bool>,
    #[prop(default = DrawerVariant::Persistent)] variant: DrawerVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerPrimitive
            mode={variant.as_str().to_string()}
            class={class}
            id={id}
            attr:data-open={move || if open.get() { "true" } else { "" }}
            attr:data-collapsed={move || if collapsed.get() { "true" } else { "" }}
        >
            {children.map(|c| c())}
        </DrawerPrimitive>
    }
}

#[component]
pub fn DrawerTrigger(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerTriggerPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </DrawerTriggerPrimitive>
    }
}

#[component]
pub fn DrawerContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerContentPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </DrawerContentPrimitive>
    }
}

#[component]
pub fn DrawerOverlay(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerOverlayPrimitive class={class} id={id} />
    }
}
