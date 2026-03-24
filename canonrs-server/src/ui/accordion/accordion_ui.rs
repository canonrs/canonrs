use leptos::prelude::*;
use canonrs_core::primitives::{
    AccordionPrimitive,
    AccordionItemPrimitive,
    AccordionTriggerPrimitive,
    AccordionContentPrimitive,
    AccordionSelection,
};
use canonrs_core::utils::id_gen::gen_accordion_item_ids;

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
    static CURRENT_ITEM_ID: std::cell::RefCell<Option<(String, String, bool)>> = std::cell::RefCell::new(None);
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] default_open: bool,
) -> impl IntoView {
    let (trigger_id, content_id) = gen_accordion_item_ids();

    CURRENT_ITEM_ID.with(|id| {
        *id.borrow_mut() = Some((trigger_id.clone(), content_id.clone(), default_open));
    });

    let content = children();

    CURRENT_ITEM_ID.with(|id| {
        *id.borrow_mut() = None;
    });

    view! {
        <AccordionItemPrimitive class={class} open={default_open}>
            {content}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let (trigger_id, content_id, default_open) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());

    view! {
        <AccordionTriggerPrimitive
            id={trigger_id}
            controls={content_id}
            class={class}
            open={default_open}
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
    let (_trigger_id, content_id, default_open) = CURRENT_ITEM_ID.with(|id| id.borrow().clone().unwrap_or_default());

    view! {
        <AccordionContentPrimitive
            id={content_id}
            class={class}
            open={default_open}
        >
            {children()}
        </AccordionContentPrimitive>
    }
}

#[component]
pub fn AccordionPreview() -> impl IntoView {
    view! {
        <Accordion id="acc-preview".to_string()>
            <AccordionItem>
                <AccordionTrigger>"Item 1"</AccordionTrigger>
                <AccordionContent>"Content"</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
