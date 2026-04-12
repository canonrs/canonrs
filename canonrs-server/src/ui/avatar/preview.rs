use leptos::prelude::*;
use super::avatar_boundary::{Avatar, AvatarImage, AvatarFallback};
use super::avatar_ui::{AvatarSize, AvatarShape, AvatarStatus};

#[component]
pub fn AvatarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <Avatar status=AvatarStatus::Online animated=true>
                        <AvatarFallback>"AB"</AvatarFallback>
                    </Avatar>
                    <Avatar>
                        <AvatarImage src="https://i.pravatar.cc/150?img=3".to_string() alt="User".to_string() />
                        <AvatarFallback>"CD"</AvatarFallback>
                    </Avatar>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Image and fallback visibility controlled by state system."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Avatar size=AvatarSize::Xs><AvatarFallback>"XS"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Sm><AvatarFallback>"SM"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Md><AvatarFallback>"MD"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Lg><AvatarFallback>"LG"</AvatarFallback></Avatar>
                    <Avatar size=AvatarSize::Xl><AvatarFallback>"XL"</AvatarFallback></Avatar>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Shapes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Avatar shape=AvatarShape::Circle><AvatarFallback>"CI"</AvatarFallback></Avatar>
                    <Avatar shape=AvatarShape::Rounded><AvatarFallback>"RO"</AvatarFallback></Avatar>
                    <Avatar shape=AvatarShape::Square><AvatarFallback>"SQ"</AvatarFallback></Avatar>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Status"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <Avatar status=AvatarStatus::Online><AvatarFallback>"ON"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Busy><AvatarFallback>"BU"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Away><AvatarFallback>"AW"</AvatarFallback></Avatar>
                    <Avatar status=AvatarStatus::Offline><AvatarFallback>"OF"</AvatarFallback></Avatar>
                </div>
            </div>
        </div>
    }
}
