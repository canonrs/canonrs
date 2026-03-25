use leptos::prelude::*;
use super::{Resizable, ResizablePanel, ResizableHandle};
use canonrs_core::primitives::ResizableOrientation;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <h4>"Horizontal Resizable"</h4>
                <div style="height: 300px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable>
                        <ResizablePanel default_size=40u32>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Left Panel (40%)"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=60u32>
                            <div style="padding: var(--space-md); height: 100%;">
                                "Right Panel (60%)"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>

            <div>
                <h4>"Vertical Resizable"</h4>
                <div style="height: 400px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable orientation=ResizableOrientation::Vertical>
                        <ResizablePanel default_size=30u32>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Top Panel (30%)"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=70u32>
                            <div style="padding: var(--space-md); height: 100%;">
                                "Bottom Panel (70%)"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>

            <div>
                <h4>"With Min/Max Constraints (20%-80%)"</h4>
                <div style="height: 300px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable min_size=20u32 max_size=80u32>
                        <ResizablePanel default_size=50u32>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Constrained Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50u32>
                            <div style="padding: var(--space-md); height: 100%;">
                                "Other Panel"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>
        </div>
    }
}
