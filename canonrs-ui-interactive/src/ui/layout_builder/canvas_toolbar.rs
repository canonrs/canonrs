use leptos::prelude::*;
use super::types::CanvasMode;

#[component]
pub fn CanvasToolbar(
    canvas_mode: RwSignal<CanvasMode>,
) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: flex-end; margin-bottom: 0.5rem; gap: 0.5rem;">
            <button
                on:click=move |_| canvas_mode.set(CanvasMode::Builder)
                style=move || format!(
                    "padding: 0.25rem 0.75rem; font-size: 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); cursor: pointer; background: {}; color: {};",
                    if canvas_mode.get() == CanvasMode::Builder { "var(--theme-action-primary-bg)" } else { "var(--theme-surface-bg)" },
                    if canvas_mode.get() == CanvasMode::Builder { "var(--theme-action-primary-fg)" } else { "var(--theme-surface-fg)" }
                )
            >"Builder"</button>
            <button
                on:click=move |_| canvas_mode.set(CanvasMode::Preview)
                style=move || format!(
                    "padding: 0.25rem 0.75rem; font-size: 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); cursor: pointer; background: {}; color: {};",
                    if canvas_mode.get() == CanvasMode::Preview { "var(--theme-action-primary-bg)" } else { "var(--theme-surface-bg)" },
                    if canvas_mode.get() == CanvasMode::Preview { "var(--theme-action-primary-fg)" } else { "var(--theme-surface-fg)" }
                )
            >"Preview"</button>
        </div>
    }
}
