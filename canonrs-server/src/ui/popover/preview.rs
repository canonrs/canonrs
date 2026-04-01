use leptos::prelude::*;
use super::popover_ui::{Popover, PopoverContent};

#[component]
pub fn PopoverShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Popover>
                    <button type="button" data-rs-popover-trigger="" data-rs-button="" data-rs-variant="primary">"Open Popover"</button>
                    <PopoverContent>
                        <div style="display:flex;flex-direction:column;gap:0.5rem;">
                            <strong>"Settings"</strong>
                            <p style="font-size:var(--font-size-sm);">"Configure your preferences here."</p>
                        </div>
                    </PopoverContent>
                </Popover>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger and content visibility governed by shared state contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With form"</span>
                <div data-rs-showcase-preview-row="">
                    <Popover>
                        <button type="button" data-rs-popover-trigger="" data-rs-button="" data-rs-variant="outline">"Filter"</button>
                        <PopoverContent>
                            <div style="display:flex;flex-direction:column;gap:0.5rem;">
                                <strong>"Filter results"</strong>
                                <p style="font-size:var(--font-size-sm);">"Apply filters to narrow your search."</p>
                            </div>
                        </PopoverContent>
                    </Popover>
                </div>
            </div>
        </div>
    }
}
