pub mod accordion_ui;
pub mod boundary;
pub mod preview;
#[cfg(feature = "examples")]
pub mod examples;

pub use accordion_ui::*;
pub use boundary::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
pub use canonrs_core::primitives::AccordionSelection;
#[cfg(feature = "examples")]
pub use examples::*;

pub use accordion_ui::AccordionPreview;
pub use preview::AccordionShowcasePreview;
