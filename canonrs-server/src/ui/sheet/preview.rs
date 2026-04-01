use leptos::prelude::*;
use super::sheet_ui::{Sheet, SheetOverlay, SheetContent};
use canonrs_core::primitives::SheetSide;

#[component]
pub fn SheetShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Sheet side=SheetSide::Right>
                    <button type="button" data-rs-sheet-trigger="" data-rs-button="" data-rs-variant="primary">"Open Sheet"</button>
                    <SheetOverlay />
                    <SheetContent aria_labelledby="sheet-title-1">
                        <h2 id="sheet-title-1">"Sheet Title"</h2>
                        <p>"Sheet content slides in from the side. State fully governed via shared visibility."</p>
                        <button type="button" data-rs-sheet-close="" data-rs-button="" data-rs-variant="outline">"Close"</button>
                    </SheetContent>
                </Sheet>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Sheet visibility and overlay fully governed via shared state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"From left"</span>
                <div data-rs-showcase-preview-row="">
                    <Sheet side=SheetSide::Left>
                        <button type="button" data-rs-sheet-trigger="" data-rs-button="" data-rs-variant="outline">"Open left"</button>
                        <SheetOverlay />
                        <SheetContent aria_labelledby="sheet-title-2">
                            <h2 id="sheet-title-2">"Navigation"</h2>
                            <p>"Left sheet for navigation or filters."</p>
                            <button type="button" data-rs-sheet-close="" data-rs-button="" data-rs-variant="outline">"Close"</button>
                        </SheetContent>
                    </Sheet>
                </div>
            </div>
        </div>
    }
}
