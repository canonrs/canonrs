use leptos::prelude::*;
use super::sheet_ui::*;
use crate::ui::button::button_ui::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Sheet>
            <Button variant=ButtonVariant::Primary attr:data-rs-sheet-trigger="">"Open Sheet"</Button>
            <SheetOverlay />
            <SheetContent aria_labelledby="sheet-example-title">
                <h2 id="sheet-example-title">"Sheet Title"</h2>
                <p>"Sheet content from the right"</p>
                <Button variant=ButtonVariant::Outline attr:data-rs-sheet-close="">"Close"</Button>
            </SheetContent>
        </Sheet>
    }
}
