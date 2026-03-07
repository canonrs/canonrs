use leptos::prelude::*;
use super::{Resizable, ResizableDirection, ResizablePanel, ResizableHandle};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Horizontal
            <div>
                <h4>"Horizontal Resizable"</h4>
                <div style="height: 300px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable id="resizable-1">
                        <ResizablePanel default_size=40>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Left Panel (40%)"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=60>
                            <div style="padding: var(--space-md); height: 100%;">
                                "Right Panel (60%)"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>

            // Vertical
            <div>
                <h4>"Vertical Resizable"</h4>
                <div style="height: 400px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable id="resizable-2" direction=ResizableDirection::Vertical>
                        <ResizablePanel default_size=30>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Top Panel (30%)"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=70>
                            <div style="padding: var(--space-md); height: 100%;">
                                "Bottom Panel (70%)"
                            </div>
                        </ResizablePanel>
                    </Resizable>
                </div>
            </div>

            // Custom constraints
            <div>
                <h4>"With Min/Max Constraints (20%-80%)"</h4>
                <div style="height: 300px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                    <Resizable id="resizable-3" min_size=20 max_size=80>
                        <ResizablePanel default_size=50>
                            <div style="padding: var(--space-md); background: var(--theme-surface-muted); height: 100%;">
                                "Constrained Panel"
                            </div>
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel default_size=50>
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
