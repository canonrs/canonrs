pub mod accordion_ui;
pub mod accordion_boundary;
pub use accordion_boundary::*;
pub mod preview;
#[cfg(feature = "examples")]
pub mod examples;

// no types to re-export from accordion_ui
pub use accordion_boundary::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
pub use canonrs_core::primitives::AccordionSelection;
#[cfg(feature = "examples")]
pub use examples::*;

pub use accordion_ui::AccordionPreview;
pub use preview::AccordionShowcasePreview;
