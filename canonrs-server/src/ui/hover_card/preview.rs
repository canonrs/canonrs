use leptos::prelude::*;
use super::hover_card_ui::{HoverCard, HoverCardTrigger, HoverCardContent};

#[component]
pub fn HoverCardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HoverCard>
                    <HoverCardTrigger>
                        <span data-rs-button="" data-rs-variant="outline">"@canonrs"</span>
                    </HoverCardTrigger>
                    <HoverCardContent>
                        <div style="display:flex;flex-direction:column;gap:0.5rem;">
                            <strong>"CanonRS"</strong>
                            <p style="font-size:var(--font-size-sm);opacity:0.7;">"A design system built in Rust and Leptos."</p>
                            <p style="font-size:var(--font-size-xs);opacity:0.5;">"Joined January 2025"</p>
                        </div>
                    </HoverCardContent>
                </HoverCard>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Hover state and positioning enforced via visibility contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Rich content"</span>
                <div data-rs-showcase-preview-row="">
                    <HoverCard>
                        <HoverCardTrigger>
                            <span data-rs-button="" data-rs-variant="ghost">"Component docs"</span>
                        </HoverCardTrigger>
                        <HoverCardContent>
                            <div style="display:flex;flex-direction:column;gap:0.5rem;">
                                <strong>"Button"</strong>
                                <p style="font-size:var(--font-size-sm);">"Triggers an action or event."</p>
                            </div>
                        </HoverCardContent>
                    </HoverCard>
                </div>
            </div>
        </div>
    }
}
