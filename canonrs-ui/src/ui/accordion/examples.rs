use leptos::prelude::*;
use super::accordion_ui::*;
use crate::primitives::AccordionSelection;

pub fn basic_example() -> impl IntoView {
    view! {
        <Accordion selection=AccordionSelection::Single collapsible=true>
            <AccordionItem>
                <AccordionTrigger>"Item 1"</AccordionTrigger>
                <AccordionContent>
                    <p>"Content of item 1."</p>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>"Item 2"</AccordionTrigger>
                <AccordionContent>
                    <p>"Content of item 2."</p>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>"Item 3"</AccordionTrigger>
                <AccordionContent>
                    <p>"Content of item 3."</p>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
