use leptos::prelude::*;
use super::avatar_island::{AvatarIsland, AvatarImageIsland, AvatarFallbackIsland};

#[component]
pub fn AvatarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland status="online" animated=true>
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
                    <AvatarIsland size="xs"><AvatarFallbackIsland>"XS"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size="sm"><AvatarFallbackIsland>"SM"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size="md"><AvatarFallbackIsland>"MD"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size="lg"><AvatarFallbackIsland>"LG"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland size="xl"><AvatarFallbackIsland>"XL"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Shapes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland shape="circle"><AvatarFallbackIsland>"CI"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland shape="rounded"><AvatarFallbackIsland>"RO"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland shape="square"><AvatarFallbackIsland>"SQ"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Status"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);">
                    <AvatarIsland status="online"><AvatarFallbackIsland>"ON"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status="busy"><AvatarFallbackIsland>"BU"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status="away"><AvatarFallbackIsland>"AW"</AvatarFallbackIsland></AvatarIsland>
                    <AvatarIsland status="offline"><AvatarFallbackIsland>"OF"</AvatarFallbackIsland></AvatarIsland>
                </div>
            </div>
        </div>
    }
}
