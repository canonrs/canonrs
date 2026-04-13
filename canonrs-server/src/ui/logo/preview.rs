use leptos::prelude::*;
use super::logo_boundary::{Logo, LogoSize, LogoVariant};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LogoShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Logo />
            <p data-rs-showcase-preview-anchor="">
                "Brand identity structure and navigation behavior enforced in a single contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Lg>
                    <Logo size=LogoSize::Sm />
                    <Logo size=LogoSize::Md />
                    <Logo size=LogoSize::Lg />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Lg>
                    <Logo variant=LogoVariant::Brand />
                    <Logo variant=LogoVariant::Neutral />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With wordmark"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Lg>
                    <Logo wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" }) />
                    <Logo
                        wordmark=leptos::children::ToChildren::to_children(|| view! { "CanonRS" })
                        tagline=leptos::children::ToChildren::to_children(|| view! { "Design System" })
                    />
                </Stack>
            </Stack>
        </Stack>
    }
}
