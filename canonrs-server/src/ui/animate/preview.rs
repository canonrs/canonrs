use leptos::prelude::*;
use super::animate_ui::Animate;
use canonrs_core::primitives::{AnimationName, AnimationEasing};

#[component]
pub fn AnimateShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Animate animation=AnimationName::FadeIn duration="1.2s".to_string()>
                    <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:160px;text-align:center;">
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
                    <Animate animation=AnimationName::FadeIn duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"FadeIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::FadeOut duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"FadeOut"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideIn duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"SlideIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideOut duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"SlideOut"</div>
                    </Animate>
                    <Animate animation=AnimationName::ScaleIn duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"ScaleIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::ScaleOut duration="1.4s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"ScaleOut"</div>
                    </Animate>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Easing"</span>
                <div data-rs-showcase-preview-row="">
                    <Animate animation=AnimationName::SlideIn easing=AnimationEasing::EaseIn duration="1.6s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseIn"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideIn easing=AnimationEasing::EaseOut duration="1.6s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseOut"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideIn easing=AnimationEasing::EaseInOut duration="1.6s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"EaseInOut"</div>
                    </Animate>
                    <Animate animation=AnimationName::SlideIn easing=AnimationEasing::Linear duration="1.6s".to_string()>
                        <div style="padding:1rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);min-width:100px;text-align:center;">"Linear"</div>
                    </Animate>
                </div>
            </div>
        </div>
    }
}
