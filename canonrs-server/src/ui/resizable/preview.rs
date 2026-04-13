use canonrs_core::primitives::ResizableOrientation;
use leptos::prelude::*;
use super::resizable_boundary::{Resizable, ResizablePanel, ResizableHandle};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ResizableShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Panel sizes constrained and behavior encoded structurally."
            </p>
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
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Vertical"</span>
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
            </Stack>
        </Stack>
    }
}
