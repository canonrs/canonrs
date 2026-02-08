use leptos::prelude::*;
use super::sheet_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <button id="open-sheet-ex" data-button data-ui-variant="default">"Open Sheet"</button>
            <Sheet id="sheet-ex".to_string() side=SheetSide::Right>
                <h2>"Sheet Title"</h2>
                <p>"Sheet content from the right side."</p>
                <button id="close-sheet-ex" data-button data-ui-variant="outline">"Close"</button>
            </Sheet>
        </div>
    }
}
