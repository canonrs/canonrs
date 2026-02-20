use leptos::prelude::*;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant};
use super::types::{BlockDef, DroppedBlock, LayoutRegion, DragContext};

#[component]
pub fn BlockPreview(
    instance_id: uuid::Uuid,
    block: BlockDef,
    region: LayoutRegion,
    dropped: RwSignal<Vec<DroppedBlock>>,
    drag_ctx: RwSignal<DragContext>,
) -> impl IntoView {
    let block_id = block.id;
    let block_label = block.label;
    let block_icon = block.icon;
    let is_dragging = move || drag_ctx.get().instance_id == Some(instance_id);

    let remove = move |_| {
        dropped.update(|v| {
            if let Some(pos) = v.iter().position(|d| d.instance_id == instance_id) {
                v.remove(pos);
            }
        });
    };

    let on_pointerdown = move |ev: leptos::ev::PointerEvent| {
        ev.prevent_default();
        drag_ctx.set(DragContext {
            instance_id: Some(instance_id),
            block_def: Some(block.clone()),
            source_region: Some(region),
        });
    };

    let inner = match block_id {
        "card" => view! {
            <Card>
                <div style="padding: 0.5rem; font-size: 0.8rem; color: var(--theme-surface-fg-muted);">"Card content"</div>
            </Card>
        }.into_any(),
        "alert" => view! {
            <AlertBlock variant=AlertVariant::Info>"Alert message"</AlertBlock>
        }.into_any(),
        _ => view! {
            <div style="padding: 0.5rem 0.75rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); font-size: 0.8rem; display: flex; align-items: center; gap: 0.5rem;">
                <span>{block_icon}</span>
                <span style="color: var(--theme-surface-fg);">{block_label}</span>
            </div>
        }.into_any(),
    };

    view! {
        <div
            style=move || format!(
                "position: relative; margin-bottom: 0.375rem; cursor: grab; opacity: {}; touch-action: none;",
                if is_dragging() { "0.4" } else { "1.0" }
            )
            on:pointerdown=on_pointerdown
        >
            {inner}
            <button
                on:click=remove
                on:pointerdown=move |ev| ev.stop_propagation()
                style="position: absolute; top: 4px; right: 4px; width: 18px; height: 18px; border-radius: 50%; border: none; background: var(--theme-destructive-bg, #ef4444); color: white; font-size: 0.65rem; cursor: pointer; display: flex; align-items: center; justify-content: center; line-height: 1; padding: 0; z-index: 10;"
            >
                "×"
            </button>
        </div>
    }
}
