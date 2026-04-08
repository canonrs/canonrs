use leptos::prelude::*;
use super::avatar_island::{AvatarIsland, AvatarImageIsland, AvatarFallbackIsland};
use super::avatar_ui::{AvatarSize, AvatarShape, AvatarStatus};

#[component]
pub fn AvatarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland status=AvatarStatus::Online animated=true>
                        <AvatarFallbackIsland>"AB"</AvatarFallbackIsland>
                    </AvatarIsland>
                    <AvatarIsland>
                        <AvatarImageIsland src="https://i.pravatar.cc/150?img=3".to_string() alt="User".to_string() />
                        <AvatarFallbackIsland>"CD"</AvatarFallbackIsland>
                    </AvatarIsland>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Image and fallback visibility controlled by state system."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland size=AvatarSize::Xs><AvatarFallbackIsland>"XS"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size=AvatarSize::Sm><AvatarFallbackIsland>"SM"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size=AvatarSize::Md><AvatarFallbackIsland>"MD"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size=AvatarSize::Lg><AvatarFallbackIsland>"LG"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size=AvatarSize::Xl><AvatarFallbackIsland>"XL"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Shapes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland shape=AvatarShape::Circle><AvatarFallbackIsland>"CI"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland shape=AvatarShape::Rounded><AvatarFallbackIsland>"RO"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland shape=AvatarShape::Square><AvatarFallbackIsland>"SQ"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Status"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland status=AvatarStatus::Online><AvatarFallbackIsland>"ON"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status=AvatarStatus::Busy><AvatarFallbackIsland>"BU"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status=AvatarStatus::Away><AvatarFallbackIsland>"AW"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status=AvatarStatus::Offline><AvatarFallbackIsland>"OF"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
        </div>
    }
}
