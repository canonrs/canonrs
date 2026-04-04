use leptos::prelude::*;
use super::link_island::{LinkIsland, LinkVariant};

#[component]
pub fn LinkShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LinkIsland href="/showcase" variant=LinkVariant::Default>
                    "View the Showcase →"
                </LinkIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation semantics and external behavior enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <LinkIsland href="#" variant=LinkVariant::Default>"Default"</LinkIsland>
                    <LinkIsland href="#" variant=LinkVariant::Muted>"Muted"</LinkIsland>
                    <LinkIsland href="#" variant=LinkVariant::Underline>"Underline"</LinkIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <LinkIsland href="#">"Default"</LinkIsland>
                    <LinkIsland href="#" disabled=true>"Disabled"</LinkIsland>
                    <LinkIsland href="https://canonrs.com" external=true>"External ↗"</LinkIsland>
                </div>
            </div>
        </div>
    }
}
