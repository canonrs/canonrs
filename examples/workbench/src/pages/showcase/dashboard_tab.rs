use leptos::prelude::*;
use rs_design::ui::dashboard::{DashboardTemplate, DashboardConfig, WidgetDef, WidgetPosition, WidgetContent};
use rs_design::ui::drag_drop::{DragHandle, DropZone, DropEvent, DragItemId, DropTargetId, DragDropCallbacks};
use rs_design::commands::use_command_history;
use rs_design::ui::dashboard::{AddWidgetCommand, RemoveWidgetCommand};

#[component]
pub fn DashboardTab() -> impl IntoView {
    let (active_template, set_active_template) = signal(DashboardTemplate::Analytics);
    let placed_widgets = RwSignal::new(Vec::<WidgetDef>::new());
    let (counter, set_counter) = signal(0);

    // ✅ Command History
    let command_history = use_command_history();

    let available_widgets = move || active_template.get().widgets();

    let callbacks = leptos::prelude::use_context::<DragDropCallbacks>()
        .expect("DragDropCallbacks context missing");

    // ✅ Callback usa COMANDO ao invés de set_placed_widgets
    let on_drop = Callback::new(move |event: DropEvent| {
        if event.target_id.0 == "dashboard-grid" {
            let widget_id = event.item_id.0.clone();

            if let Some(template_widget) = active_template.get().widgets().into_iter().find(|w| w.id == widget_id) {
                let unique_id = format!("{}-{}", widget_id, counter.get());
                set_counter.update(|c| *c += 1);

                let new_widget = WidgetDef {
                    id: unique_id.clone(),
                    title: template_widget.title.clone(),
                    position: WidgetPosition::new(0, placed_widgets.get().len(), template_widget.position.width, template_widget.position.height),
                    content: template_widget.content,
                    removable: true,
                    resizable: false,
                };

                // ✅ COMANDO ao invés de mutação direta
                command_history.execute(AddWidgetCommand {
                    widgets: placed_widgets,
                    widget: new_widget,
                });
            }
        }
    });

    callbacks.register_drop(on_drop);

    // ✅ Callback de remoção usa COMANDO
    let on_widget_remove = Callback::new(move |widget_id: String| {
        let widgets_vec = placed_widgets.get();
        if let Some((index, widget)) = widgets_vec.iter().enumerate().find(|(_, w)| w.id == widget_id) {
            command_history.execute(RemoveWidgetCommand {
                widgets: placed_widgets,
                widget: widget.clone(),
                index,
            });
        }
    });

    let config = DashboardConfig::new().columns(12).row_height(80).gap(16);

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Dashboard Layout"</h2>
                <p class="text-sm text-muted-foreground">"Drag widgets + Undo/Redo (Ctrl+Z / Ctrl+Y)"</p>
            </div>

            {/* BOTÕES DE TEMPLATE */}
            <div class="flex gap-2 border-b pb-4">
                <button
                    class=move || if active_template.get() == DashboardTemplate::Analytics {
                        "px-4 py-2 text-sm font-medium border rounded-md bg-primary text-primary-foreground"
                    } else {
                        "px-4 py-2 text-sm font-medium border rounded-md hover:bg-muted"
                    }
                    on:click=move |_| set_active_template.set(DashboardTemplate::Analytics)
                >
                    "📊 Analytics"
                </button>

                <button
                    class=move || if active_template.get() == DashboardTemplate::Sales {
                        "px-4 py-2 text-sm font-medium border rounded-md bg-primary text-primary-foreground"
                    } else {
                        "px-4 py-2 text-sm font-medium border rounded-md hover:bg-muted"
                    }
                    on:click=move |_| set_active_template.set(DashboardTemplate::Sales)
                >
                    "💰 Sales"
                </button>

                <button
                    class=move || if active_template.get() == DashboardTemplate::DevOps {
                        "px-4 py-2 text-sm font-medium border rounded-md bg-primary text-primary-foreground"
                    } else {
                        "px-4 py-2 text-sm font-medium border rounded-md hover:bg-muted"
                    }
                    on:click=move |_| set_active_template.set(DashboardTemplate::DevOps)
                >
                    "🚀 DevOps"
                </button>

                <button
                    class=move || if active_template.get() == DashboardTemplate::Executive {
                        "px-4 py-2 text-sm font-medium border rounded-md bg-primary text-primary-foreground"
                    } else {
                        "px-4 py-2 text-sm font-medium border rounded-md hover:bg-muted"
                    }
                    on:click=move |_| set_active_template.set(DashboardTemplate::Executive)
                >
                    "👔 Executive"
                </button>

                {/* UNDO/REDO UI */}
                <div class="ml-auto flex gap-2">
                    <button
                        class="px-3 py-2 text-sm border rounded-md hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || !command_history.can_undo().get()
                        on:click=move |_| { command_history.undo(); }
                        title="Undo (Ctrl+Z)"
                    >
                        "↩️ Undo"
                    </button>
                    <button
                        class="px-3 py-2 text-sm border rounded-md hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || !command_history.can_redo().get()
                        on:click=move |_| { command_history.redo(); }
                        title="Redo (Ctrl+Y)"
                    >
                        "↪️ Redo"
                    </button>
                </div>
            </div>

            {/* WIDGETS DISPONÍVEIS */}
            <div class="border rounded-lg p-4 bg-muted/20">
                <h3 class="font-semibold mb-3">{move || format!("{} Widgets", active_template.get().name())}</h3>
                <div class="flex gap-3 flex-wrap">
                    {move || available_widgets().into_iter().map(|widget| {
                        view! {
                            <DragHandle item_id=DragItemId::new(widget.id.clone()) data=String::new()>
                                <div class="px-4 py-3 border rounded-lg cursor-grab hover:bg-muted hover:border-primary transition-all bg-card">
                                    <span class="text-sm font-medium">{widget.title}</span>
                                </div>
                            </DragHandle>
                        }
                    }).collect_view()}
                </div>
            </div>

            {/* ÁREA DE MONTAGEM */}
            <div class="border rounded-lg p-4">
                <h3 class="font-semibold mb-4">{move || format!("{} Dashboard", active_template.get().name())}</h3>

                <DropZone
                    target_id=DropTargetId::new("dashboard-grid")
                    class="min-h-[500px] border-2 border-dashed rounded-lg p-4"
                    hover_class="border-primary bg-primary/5"
                >
                    {move || {
                        let widgets = placed_widgets.get();
                        if widgets.is_empty() {
                            view! {
                                <div class="flex items-center justify-center h-[450px]">
                                    <div class="text-center text-muted-foreground">
                                        <div class="text-6xl mb-4">"📊"</div>
                                        <p class="text-lg font-semibold">"Empty Dashboard"</p>
                                        <p class="text-sm">"Drag widgets from above"</p>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div
                                    class="grid gap-4"
                                    style=format!("grid-template-columns: repeat({}, 1fr);", config.columns)
                                >
                                    {widgets.into_iter().map(|widget| {
                                        let pos = widget.position.clone();
                                        let id = widget.id.clone();
                                        let content = widget.content.clone();

                                        view! {
                                            <div
                                                class="border rounded-lg bg-card shadow-sm"
                                                style=format!(
                                                    "grid-column: {} / span {}; grid-row: {} / span {}; min-height: {}px;",
                                                    pos.x + 1, pos.width, pos.y + 1, pos.height, pos.height * config.row_height
                                                )
                                            >
                                                <div class="flex items-center justify-between p-3 border-b bg-muted/30">
                                                    <h3 class="font-semibold text-sm">{widget.title}</h3>
                                                    <button
                                                        class="text-xs px-2 py-1 hover:bg-destructive/10 hover:text-destructive rounded"
                                                        on:click=move |_| {
                                                            on_widget_remove.run(id.clone());
                                                        }
                                                    >
                                                        "✕"
                                                    </button>
                                                </div>
                                                <div class="p-4">
                                                    {match content {
                                                        WidgetContent::Custom(render) => render(),
                                                        WidgetContent::Empty => view! { <div class="text-center text-muted-foreground">"Empty"</div> }.into_any(),
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </DropZone>
            </div>
        </div>
    }
}
