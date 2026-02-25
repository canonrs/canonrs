use leptos::prelude::*;
use super::builder_workspace::BuilderWorkspace;
use crate::application::builder_controller::BuilderController;
use crate::ui::theme_workspace::theme_workspace::ThemeWorkspace;
use crate::infra::app_state::*;

#[component]
pub fn LayoutBuilderInteractive() -> impl IntoView {
    let mode = global_workspace_mode();
    let theme = global_theme();
    let engine = global_engine();
    let tree = global_tree();
    let selected_id = global_selected();
    let is_dirty = global_dirty();
    let active_layout = global_active_layout();
    let slots = global_slots();
    let drag_ctx = global_drag_ctx();
    let canvas_mode = global_canvas_mode();
    let drag_visual = global_drag_visual();

    let controller = BuilderController::new(engine, tree, selected_id, is_dirty);

    view! {
        <div style="display:flex;flex-direction:column;height:100%;min-height:600px;border:1px solid var(--theme-surface-border);border-radius:var(--radius-md);overflow:hidden;">
            <div style="display:flex;gap:0.5rem;padding:0.4rem 1rem;border-bottom:1px solid var(--theme-surface-border);background:var(--theme-surface-bg);flex-shrink:0;">
                <button
                    on:click=move |_| mode.set(WorkspaceMode::Builder)
                    style=move || format!("padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:{};color:{};",
                        if mode.get()==WorkspaceMode::Builder {"var(--theme-primary-bg)"} else {"transparent"},
                        if mode.get()==WorkspaceMode::Builder {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
                    )>"🧱 Builder"</button>
                <button
                    on:click=move |_| mode.set(WorkspaceMode::Theme)
                    style=move || format!("padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:{};color:{};",
                        if mode.get()==WorkspaceMode::Theme {"var(--theme-primary-bg)"} else {"transparent"},
                        if mode.get()==WorkspaceMode::Theme {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
                    )>"🎨 Theme Engine"</button>
            </div>
            <div style=move || if mode.get()==WorkspaceMode::Theme { "flex:1;overflow:hidden;" } else { "display:none;" }>
                <ThemeWorkspace controller=controller theme=theme.clone() />
            </div>
            <div style=move || if mode.get()==WorkspaceMode::Builder { "flex:1;overflow:hidden;" } else { "display:none;" }>
                <BuilderWorkspace
                    controller=controller
                    active_layout=active_layout
                    slots=slots
                    tree=tree
                    engine=engine
                    drag_ctx=drag_ctx
                    canvas_mode=canvas_mode
                    drag_visual=drag_visual
                />
            </div>
        </div>
    }
}
