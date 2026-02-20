use leptos::prelude::*;
use crate::primitives::{
    AccordionPrimitive,
    AccordionItemPrimitive,
    AccordionTriggerPrimitive,
    AccordionContentPrimitive,
    AccordionSelection,
};
use crate::utils::id_gen::gen_accordion_item_ids;

#[component]
pub fn Accordion(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <AccordionPrimitive
            selection={selection}
            collapsible={collapsible}
            class={class}
            id={id}
        >
            {children()}
        </AccordionPrimitive>
    }
}

thread_local! {
    static CURRENT_ITEM_ID: std::cell::RefCell<Option<(String, String)>> = std::cell::RefCell::new(None);
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let (trigger_id, content_id) = gen_accordion_item_ids();

    CURRENT_ITEM_ID.with(|id| {
        *id.borrow_mut() = Some((trigger_id, content_id));
    });

    let content = children();

    CURRENT_ITEM_ID.with(|id| {
        *id.borrow_mut() = None;
    });

    view! {
        <AccordionItemPrimitive class={class}>
            {content}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let (trigger_id, content_id) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());

    view! {
        <AccordionTriggerPrimitive
            id={trigger_id}
            controls={content_id}
            class={class}
        >
            {children()}
        </AccordionTriggerPrimitive>
    }
}

#[component]
pub fn AccordionContent(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let (_trigger_id, content_id) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());

    view! {
        <AccordionContentPrimitive
            id={content_id}
            class={class}
        >
            {children()}
        </AccordionContentPrimitive>
    }
}
