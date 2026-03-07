use leptos::prelude::*;
use super::AspectRatio;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem; max-width: 600px;">
            <div>
                <h4>"16:9 (Default - Video)"</h4>
                <AspectRatio>
                    <div style="background: var(--theme-surface-muted); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        "16:9 Video"
                    </div>
                </AspectRatio>
            </div>

            <div>
                <h4>"4:3 (Classic)"</h4>
                <AspectRatio width=4.0 height=3.0>
                    <div style="background: var(--theme-surface-muted); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        "4:3 Classic"
                    </div>
                </AspectRatio>
            </div>

            <div>
                <h4>"1:1 (Square)"</h4>
                <AspectRatio width=1.0 height=1.0>
                    <div style="background: var(--theme-surface-muted); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        "1:1 Square"
                    </div>
                </AspectRatio>
            </div>

            <div>
                <h4>"21:9 (Ultra-wide)"</h4>
                <AspectRatio width=21.0 height=9.0>
                    <div style="background: var(--theme-surface-muted); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        "21:9 Ultra-wide"
                    </div>
                </AspectRatio>
            </div>

            <div>
                <h4>"3:2 (Photography)"</h4>
                <AspectRatio width=3.0 height=2.0>
                    <div style="background: var(--theme-surface-muted); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        "3:2 Photo"
                    </div>
                </AspectRatio>
            </div>
        </div>
    }
}
