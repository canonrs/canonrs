use leptos::prelude::*;
use super::link_boundary::Link;
use canonrs_core::primitives::LinkVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LinkShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Link href="/showcase" variant=LinkVariant::Default>"View the Showcase →"</Link>
            <p data-rs-showcase-preview-anchor="">
                "Navigation semantics and external behavior enforced structurally."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Link href="#" variant=LinkVariant::Default>"Default"</Link>
                    <Link href="#" variant=LinkVariant::Muted>"Muted"</Link>
                    <Link href="#" variant=LinkVariant::Underline>"Underline"</Link>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Link href="#">"Default"</Link>
                    <Link href="#" disabled=true>"Disabled"</Link>
                    <Link href="https://canonrs.com" external=true>"External ↗"</Link>
                </Stack>
            </Stack>
        </Stack>
    }
}
