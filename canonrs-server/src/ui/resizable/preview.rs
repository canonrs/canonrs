use canonrs_core::primitives::ResizableOrientation;
use leptos::prelude::*;
use super::resizable_boundary::{Resizable, ResizablePanel, ResizableHandle};

#[component]
pub fn ResizableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;padding:0;">
                <div style="width:100%;height:280px;display:flex;">
                    <Resizable>
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);height:100%;display:flex;align-items:center;justify-content:center;border-right:1px solid rgba(255,255,255,0.06);overflow:hidden;">
                                "Left Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);height:100%;display:flex;align-items:center;justify-content:center;overflow:hidden;">
                                "Right Panel"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Panel sizes constrained and behavior encoded structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="" style="width:100%;height:280px;display:flex;flex-direction:column;">
                    <Resizable orientation=ResizableOrientation::Vertical>
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);width:100%;display:flex;align-items:center;justify-content:center;border-bottom:1px solid rgba(255,255,255,0.06);overflow:hidden;">
                                "Top Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);width:100%;display:flex;align-items:center;justify-content:center;overflow:hidden;">
                                "Bottom Panel"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>
        </div>
    }
}
