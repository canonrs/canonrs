use leptos::prelude::*;
use super::{Pulse, PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <p class="text-sm font-semibold mb-2">"Variants"</p>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse variant=PulseVariant::Subtle />
                        <span>"Subtle"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse variant=PulseVariant::Default />
                        <span>"Default"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse variant=PulseVariant::Emphasized />
                        <span>"Emphasized"</span>
                    </div>
                </div>
            </div>

            <div>
                <p class="text-sm font-semibold mb-2">"Sizes"</p>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse size=PulseSize::Small />
                        <span>"Small"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse size=PulseSize::Medium />
                        <span>"Medium"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse size=PulseSize::Large />
                        <span>"Large"</span>
                    </div>
                </div>
            </div>

            <div>
                <p class="text-sm font-semibold mb-2">"Speeds"</p>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse speed=PulseSpeed::Slow />
                        <span>"Slow"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse speed=PulseSpeed::Normal />
                        <span>"Normal"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Pulse speed=PulseSpeed::Fast />
                        <span>"Fast"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
