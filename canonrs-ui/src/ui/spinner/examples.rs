use leptos::prelude::*;
use super::{Spinner, SpinnerSize};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <p class="text-sm font-semibold mb-2">"Sizes (Active)"</p>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Small />
                        <span>"Small"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Medium />
                        <span>"Medium"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Large />
                        <span>"Large"</span>
                    </div>
                </div>
            </div>

            <div>
                <p class="text-sm font-semibold mb-2">"Paused State"</p>
                <div style="display: flex; align-items: center; gap: 1.5rem;">
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Small paused=true />
                        <span>"Small (Paused)"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Medium paused=true />
                        <span>"Medium (Paused)"</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <Spinner size=SpinnerSize::Large paused=true />
                        <span>"Large (Paused)"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
