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
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AccordionPrimitive
            selection={selection}
            collapsible={collapsible}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
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
    #[prop(optional)] class: Option<String>,
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
        <AccordionItemPrimitive class={class.unwrap_or_default()}>
            {content}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (trigger_id, content_id) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());
    
    view! {
        <button
            data-accordion-trigger={content_id.clone()}
            type="button"
            aria-expanded="false"
            class={class.unwrap_or_default()}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn AccordionContent(
    children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (trigger_id, content_id) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());
    
    view! {
        <div
            data-accordion-content={content_id}
            aria-hidden="true"
            hidden=true
            class={class.unwrap_or_default()}
        >
            {children()}
        </div>
    }
}
