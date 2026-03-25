//! @canon-level: api
//! Accordion API Contract

/// Accordion root component
///
/// Props:
/// - `selection: AccordionSelection` — Single (default) or Multiple
/// - `collapsible: bool` — Allow closing open item (default: true)
/// - `class: String` — CSS class
/// - `id: Option<String>` — Optional HTML id
pub struct AccordionApi;

/// AccordionItem props:
/// - `class: String`
/// - `default_open: bool` — Open on mount (default: false)
pub struct AccordionItemApi;

/// AccordionTrigger props:
/// - `class: String`
/// - `children: Children` — Button label content
pub struct AccordionTriggerApi;

/// AccordionContent props:
/// - `class: String`
/// - `children: Children` — Collapsible content
pub struct AccordionContentApi;
