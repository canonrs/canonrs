use leptos::prelude::*;
use super::loading_overlay_island::LoadingOverlayIsland;

#[component]
pub fn LoadingOverlayShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <LoadingOverlayIsland state="loading">
                    <div style="padding:var(--space-lg);min-height:120px;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:var(--space-sm);">
                        <span style="font-size:var(--font-size-sm);color:var(--theme-surface-fg);">"Título do card"</span>
                        <span style="font-size:var(--font-size-xs);color:var(--theme-surface-fg-muted);">"Descrição do conteúdo"</span>
                    </div>
                </LoadingOverlayIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Loading visibility and aria-busy managed automatically."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <LoadingOverlayIsland>
                        <div style="padding:var(--space-md);min-height:72px;display:flex;align-items:center;justify-content:center;">
                            "Idle — conteúdo visível"
                        </div>
                    </LoadingOverlayIsland>
                    <LoadingOverlayIsland state="loading">
                        <div style="padding:var(--space-md);min-height:72px;display:flex;align-items:center;justify-content:center;">
                            "Loading — conteúdo bloqueado"
                        </div>
                    </LoadingOverlayIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Modes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);width:100%;">
                    <LoadingOverlayIsland state="loading" mode="blocking">
                        <div style="padding:var(--space-md);min-height:72px;display:flex;align-items:center;justify-content:center;">
                            "Blocking"
                        </div>
                    </LoadingOverlayIsland>
                    <LoadingOverlayIsland state="loading" mode="subtle">
                        <div style="padding:var(--space-md);min-height:72px;display:flex;align-items:center;justify-content:center;">
                            "Transparent"
                        </div>
                    </LoadingOverlayIsland>
                </div>
            </div>
        </div>
    }
}
