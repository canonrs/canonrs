use leptos::prelude::*;
use super::avatar_boundary::{Avatar, AvatarImage, AvatarFallback};
use super::avatar_boundary::{AvatarSize, AvatarShape, AvatarStatus};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn AvatarShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                <Avatar status=AvatarStatus::Online>
                    <AvatarFallback>"AB"</AvatarFallback>
                </Avatar>
                <Avatar shape=AvatarShape::Circle size=AvatarSize::Lg>
                    <AvatarImage src="/assets/avatar_canonrs.svg".to_string() alt="User".to_string() />
                    <AvatarFallback>"CD"</AvatarFallback>
                </Avatar>
            </Stack>
            <p data-rs-showcase-preview-anchor="">
                "Image and fallback visibility controlled by state system."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Avatar size=AvatarSize::Xs><AvatarFallback>"XS"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Sm><AvatarFallback>"SM"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Md><AvatarFallback>"MD"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Lg><AvatarFallback>"LG"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Xl><AvatarFallback>"XL"</AvatarFallback></Avatar>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Shapes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Avatar shape=AvatarShape::Circle> <AvatarFallback>"CI"</AvatarFallback></Avatar>
                    <Avatar shape=AvatarShape::Rounded><AvatarFallback>"RO"</AvatarFallback></Avatar>
                    <Avatar shape=AvatarShape::Square> <AvatarFallback>"SQ"</AvatarFallback></Avatar>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Status"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <Avatar status=AvatarStatus::Online> <AvatarFallback>"ON"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Busy>   <AvatarFallback>"BU"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Away>   <AvatarFallback>"AW"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Offline><AvatarFallback>"OF"</AvatarFallback></Avatar>
                </Stack>
            </Stack>
        </Stack>
    }
}
