use leptos::prelude::*;
use super::resizable_ui::{Resizable, ResizablePanel, ResizableHandle};
use super::ResizableOrientation;

#[component]
pub fn ResizableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <div style="width:100%;height:300px;display:flex;">
                    <Resizable orientation=ResizableOrientation::Horizontal>
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);height:100%;display:flex;align-items:center;justify-content:center;">
                                "Left Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);height:100%;display:flex;align-items:center;justify-content:center;">
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
                <div data-rs-showcase-preview-row="" style="width:100%;height:300px;display:flex;">
                    <Resizable orientation=ResizableOrientation::Vertical>
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);display:flex;align-items:center;justify-content:center;">
                                "Top Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50u32>
                            <div style="padding:var(--space-md);display:flex;align-items:center;justify-content:center;">
                                "Bottom Panel"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>
        </div>
    }
}
