use leptos::prelude::*;
use super::icon_boundary::{Icon, IconSize, IconVariant};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn IconShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                <Icon size=IconSize::Lg variant=IconVariant::Primary>"★"</Icon>
                <Icon size=IconSize::Lg>"★"</Icon>
                <Icon size=IconSize::Lg variant=IconVariant::Muted>"★"</Icon>
            </Stack>
            <p data-rs-showcase-preview-anchor="">
                "Icon size and variant enforced via typed enums."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Icon size=IconSize::Sm>"★"</Icon>
                    <Icon size=IconSize::Md>"★"</Icon>
                    <Icon size=IconSize::Lg>"★"</Icon>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Icon>"★"</Icon>
                    <Icon variant=IconVariant::Muted>"★"</Icon>
                    <Icon variant=IconVariant::Primary>"★"</Icon>
                    <Icon variant=IconVariant::Destructive>"★"</Icon>
                    <Icon variant=IconVariant::Success>"★"</Icon>
                    <Icon variant=IconVariant::Warning>"★"</Icon>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Spin"</span>
                <Icon spin=true>"⟳"</Icon>
            </Stack>
        </Stack>
    }
}
