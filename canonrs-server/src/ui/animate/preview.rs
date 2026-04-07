use leptos::prelude::*;
use super::animate_island::AnimateIsland;

#[component]
pub fn AnimateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AnimateIsland animation="fade-in" duration="1.2s">
                    <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:160px;text-align:center;">
                        "Fade In"
                    </div>
                </AnimateIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Animation type and easing enforced through typed enums. Respects prefers-reduced-motion. Pauses when off-screen."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Animation variants"</span>
                <div data-rs-showcase-preview-row="">
                    <AnimateIsland animation="fade-in" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"FadeIn"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="fade-out" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"FadeOut"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="slide-in" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"SlideIn"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="slide-out" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"SlideOut"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="scale-in" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"ScaleIn"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="scale-out" duration="1.4s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"ScaleOut"</div>
                    </AnimateIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Easing"</span>
                <div data-rs-showcase-preview-row="">
                    <AnimateIsland animation="slide-in" easing="ease-in" duration="1.6s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseIn"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="slide-in" easing="ease-out" duration="1.6s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseOut"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="slide-in" easing="ease-in-out" duration="1.6s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseInOut"</div>
                    </AnimateIsland>
                    <AnimateIsland animation="slide-in" easing="linear" duration="1.6s">
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"Linear"</div>
                    </AnimateIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Stagger"</span>
                <div data-rs-showcase-preview-row="">
                    <AnimateIsland animation="fade-in" duration="0.6s" stagger=100.0>
                        <div style="padding:0.75rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:80px;text-align:center;">"Item 1"</div>
                        <div style="padding:0.75rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:80px;text-align:center;">"Item 2"</div>
                        <div style="padding:0.75rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:80px;text-align:center;">"Item 3"</div>
                    </AnimateIsland>
                </div>
            </div>
        </div>
    }
}
