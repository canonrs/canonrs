use leptos::prelude::*;
use super::boundary::Link;
use canonrs_core::primitives::LinkVariant;

#[component]
pub fn LinkShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Link href="/showcase" variant=LinkVariant::Default>
                    "View the Showcase →"
                </Link>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation semantics and external behavior enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Link href="#" variant=LinkVariant::Default>"Default"</Link>
                    <Link href="#" variant=LinkVariant::Muted>"Muted"</Link>
                    <Link href="#" variant=LinkVariant::Underline>"Underline"</Link>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <Link href="#">"Default"</Link>
                    <Link href="#" disabled=true>"Disabled"</Link>
                    <Link href="https://canonrs.com" external=true>"External ↗"</Link>
                </div>
            </div>
        </div>
    }
}
