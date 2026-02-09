use leptos::prelude::*;
use super::sheet_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <SheetTrigger target_sheet_id="sheet-ex">
                <button data-button data-ui-variant="solid">"Open Sheet"</button>
            </SheetTrigger>
            <Sheet id="sheet-ex" side=SheetSide::Right>
                <SheetOverlay />
                <SheetContent>
                    <h2>"Sheet Title"</h2>
                    <p>"Sheet content from the right"</p>
                    <button data-button data-ui-variant="outline" onclick="document.getElementById('sheet-ex').setAttribute('data-state', 'closed')">"Close"</button>
                </SheetContent>
            </Sheet>
        </div>
    }
}
