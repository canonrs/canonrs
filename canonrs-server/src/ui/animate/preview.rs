use leptos::prelude::*;
use super::animate_ui::Animate;
use canonrs_core::primitives::{AnimationName, AnimationEasing};

#[component]
pub fn AnimateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Animate animation=AnimationName::FadeIn>
                    <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">
                        "Fade In"
                    </div>
                </Animate>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Animation type and easing enforced through typed enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Animation variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Animate animation=AnimationName::FadeIn>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"FadeIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideIn>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"SlideIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::ScaleIn>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"ScaleIn"</div>
                    </Animate>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Easing"</span>
                <div data-rs-showcase-preview-row="">
                    <Animate animation=AnimationName::FadeIn easing=AnimationEasing::EaseIn>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"EaseIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::FadeIn easing=AnimationEasing::EaseOut>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"EaseOut"</div>
                    </Animate>
                    <Animate animation=AnimationName::FadeIn easing=AnimationEasing::Linear>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"Linear"</div>
                    </Animate>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Duration + delay"</span>
                <div data-rs-showcase-preview-row="">
                    <Animate animation=AnimationName::FadeIn duration="200ms".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"200ms"</div>
                    </Animate>
                    <Animate animation=AnimationName::FadeIn duration="600ms".to_string() delay="300ms".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);">"600ms + 300ms delay"</div>
                    </Animate>
                </div>
            </div>
        </div>
    }
}
