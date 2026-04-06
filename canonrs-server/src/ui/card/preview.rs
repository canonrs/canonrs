use leptos::prelude::*;
use super::card_island::{CardIsland, CardHeaderIsland, CardTitleIsland, CardDescriptionIsland, CardContentIsland, CardFooterIsland};

#[component]
pub fn CardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CardIsland>
                    <CardHeaderIsland>
                        <CardTitleIsland>"Getting Started"</CardTitleIsland>
                        <CardDescriptionIsland>"Everything you need to build with CanonRS."</CardDescriptionIsland>
                    </CardHeaderIsland>
                    <CardContentIsland>
                        <p>"Card structure enforced with defined regions and roles."</p>
                    </CardContentIsland>
                    <CardFooterIsland>
                        <span>"Last updated: today"</span>
                    </CardFooterIsland>
                </CardIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Card structure enforced with defined regions and roles."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-md);width:100%;">
                    <CardIsland>
                        <CardHeaderIsland>
                            <CardTitleIsland>"Header only"</CardTitleIsland>
                        </CardHeaderIsland>
                    </CardIsland>
                    <CardIsland>
                        <CardContentIsland>
                            <p>"Content only — no header or footer."</p>
                        </CardContentIsland>
                    </CardIsland>
                    <CardIsland>
                        <CardHeaderIsland>
                            <CardTitleIsland>"Full card"</CardTitleIsland>
                            <CardDescriptionIsland>"With all three regions."</CardDescriptionIsland>
                        </CardHeaderIsland>
                        <CardContentIsland>
                            <p>"Body content goes here."</p>
                        </CardContentIsland>
                        <CardFooterIsland>
                            <span>"Footer action"</span>
                        </CardFooterIsland>
                    </CardIsland>
                </div>
            </div>
        </div>
    }
}
