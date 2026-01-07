use leptos::prelude::*;
use super::types::{WidgetDef, DashboardConfig, WidgetDragEvent, WidgetRemoveEvent, WidgetPosition};
use crate::ui::drag_drop::{DragHandle, DropZone, DropEvent, DragItemId, DropTargetId};

#[component]
pub fn Dashboard(
    widgets: Vec<WidgetDef>,
    #[prop(optional)] config: Option<DashboardConfig>,
    #[prop(optional)] on_widget_drag: Option<Callback<WidgetDragEvent>>,
    #[prop(optional)] on_widget_remove: Option<Callback<WidgetRemoveEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let config = config.unwrap_or_default();

    let on_drag_stored = StoredValue::new(on_widget_drag);

    let on_drop = Callback::new(move |event: DropEvent| {
        leptos::logging::log!("📦 Drop: {} → {}", event.item_id.0, event.target_id.0);

        if let Some(cb) = on_drag_stored.get_value() {
            cb.run(WidgetDragEvent {
                widget_id: event.item_id.0.clone(),
                from_position: WidgetPosition::new(0, 0, 1, 1),
                to_position: WidgetPosition::new(0, 0, 1, 1),
            });
        }
    });

    let on_remove_stored = StoredValue::new(on_widget_remove);

    view! {
            <DropZone
                target_id=DropTargetId::new("dashboard-grid")
                class="min-h-[600px] border-2 border-dashed border-gray-300 rounded-lg p-4"
                hover_class="border-primary bg-primary/5"
            >
                <div 
                    class=format!("dashboard-grid grid gap-4 {}", class)
                    style=format!("grid-template-columns: repeat({}, 1fr);", config.columns)
                >
                    {widgets.into_iter().map(|widget| {
                        let position = widget.position.clone();

                        let widget_id = widget.id.clone();
                        let on_remove_cb = Callback::new(move |_: ()| {
                            if let Some(cb) = on_remove_stored.get_value() {
                                cb.run(WidgetRemoveEvent {
                                    widget_id: widget_id.clone(),
                                });
                            }
                        });

                        view! {
                            <DragHandle item_id=DragItemId::new(widget.id.clone())>
                                <div
                                    class="dashboard-widget border rounded-lg bg-card shadow-sm hover:shadow-md transition-all"
                                    style=format!(
                                        "grid-column: {} / span {}; grid-row: {} / span {}; min-height: {}px;",
                                        position.x + 1, position.width, position.y + 1, position.height,
                                        position.height * config.row_height
                                    )
                                >
                                    <div class="widget-header flex items-center justify-between p-3 border-b bg-muted/30 cursor-grab">
                                        <h3 class="font-semibold text-sm">{widget.title.clone()}</h3>
                                        {if widget.removable {
                                            view! {
                                                <button
                                                    class="text-xs text-muted-foreground hover:text-destructive px-2 py-1 hover:bg-destructive/10 rounded"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        on_remove_cb.run(());
                                                    }
                                                >
                                                    "✕"
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }}
                                    </div>

                                    <div class="widget-content p-4 overflow-auto">
                                        {match widget.content {
                                            super::types::WidgetContent::Custom(render_fn) => render_fn(),
                                            super::types::WidgetContent::Empty => view! {
                                                <div class="text-sm text-muted-foreground text-center py-8">
                                                    "Empty widget"
                                                </div>
                                            }.into_any(),
                                        }}
                                    </div>
                                </div>
                            </DragHandle>
                        }
                    }).collect_view()}
                </div>
            </DropZone>
    }
}
