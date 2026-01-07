use leptos::prelude::*;
use rs_design::ui::drag_drop::{SortableList, DropZone, DragHandle,
    DragStartEvent, DropEvent, DragItemId, DropTargetId
};

#[derive(Clone, Debug, PartialEq)]
struct Task {
    id: String,
    label: String,
    color: String,
}

#[component]
pub fn DragDropTab() -> impl IntoView {
    let (zone_a, set_zone_a) = signal(vec![
        Task { id: "1".into(), label: "Task A".into(), color: "bg-blue-100 text-blue-800".into() },
        Task { id: "2".into(), label: "Task B".into(), color: "bg-green-100 text-green-800".into() },
        Task { id: "3".into(), label: "Task C".into(), color: "bg-purple-100 text-purple-800".into() },
    ]);

    let (zone_b, set_zone_b) = signal(vec![
        Task { id: "4".into(), label: "Task D".into(), color: "bg-orange-100 text-orange-800".into() },
        Task { id: "5".into(), label: "Task E".into(), color: "bg-pink-100 text-pink-800".into() },
    ]);

    let (log, set_log) = signal(Vec::<String>::new());

    let on_drag_start = Callback::new(move |event: DragStartEvent| {
        let msg = format!("🎯 Drag started: {}", event.item_id.0);
        set_log.update(|l| l.insert(0, msg));
    });

    // Global drop handler - move entre zonas
    let on_drop = Callback::new(move |event: DropEvent| {
        let item_id = event.item_id.0.clone();
        let target_zone = event.target_id.0.clone();
        
        let msg = format!("📦 Dropped {} on {}", item_id, target_zone);
        set_log.update(|l| {
            l.insert(0, msg);
            if l.len() > 10 { l.truncate(10); }
        });

        // Move item entre zonas
        let mut moved_task: Option<Task> = None;

        // Remove from zone A
        set_zone_a.update(|tasks| {
            if let Some(pos) = tasks.iter().position(|t| t.id == item_id) {
                moved_task = Some(tasks.remove(pos));
            }
        });

        // If not in A, remove from zone B
        if moved_task.is_none() {
            set_zone_b.update(|tasks| {
                if let Some(pos) = tasks.iter().position(|t| t.id == item_id) {
                    moved_task = Some(tasks.remove(pos));
                }
            });
        }

        // Add to target zone
        if let Some(task) = moved_task {
            if target_zone == "zone-a" {
                set_zone_a.update(|tasks| tasks.push(task));
            } else if target_zone == "zone-b" {
                set_zone_b.update(|tasks| tasks.push(task));
            }
        }
    });

    let on_reorder_a = Callback::new(move |new_tasks: Vec<Task>| {
        set_zone_a.set(new_tasks);
        set_log.update(|l| {
            l.insert(0, "✨ Zone A reordered".to_string());
            if l.len() > 10 { l.truncate(10); }
        });
    });

    let on_reorder_b = Callback::new(move |new_tasks: Vec<Task>| {
        set_zone_b.set(new_tasks);
        set_log.update(|l| {
            l.insert(0, "✨ Zone B reordered".to_string());
            if l.len() > 10 { l.truncate(10); }
        });
    });

    view! {
            <div class="space-y-6">
                <div>
                    <h2 class="text-2xl font-bold mb-2">"Drag & Drop: Multi-Zone"</h2>
                    <p class="text-muted-foreground">"Drag within zone to reorder, or drag to zone header to move between zones"</p>
                </div>

                <div class="grid grid-cols-3 gap-6">
                    <div class="col-span-2">
                        <div class="grid grid-cols-2 gap-6">
                            // Zone A
                            <div class="border-2 border-dashed border-blue-300 rounded-lg overflow-hidden min-h-[400px]">
                                <DropZone
                                    target_id=DropTargetId::new("zone-a")
                                    class="p-4 h-full"
                                    hover_class="bg-blue-50"
                                >
                                    <h3 class="font-semibold mb-4 text-blue-900">"Zone A"</h3>
                                    
                                    <SortableList
                                        items=zone_a
                                        on_reorder=on_reorder_a
                                        item_id=|task: &Task| task.id.clone()
                                        render=move |task: Task| {
                                            view! {
                                                <div class=format!("p-3 rounded-lg shadow-sm cursor-grab active:cursor-grabbing {}", task.color)>
                                                    <span class="font-medium">{task.label}</span>
                                                </div>
                                            }
                                        }
                                    />
                                </DropZone>
                            </div>

                            // Zone B
                            <div class="border-2 border-dashed border-green-300 rounded-lg overflow-hidden min-h-[400px]">
                                <DropZone
                                    target_id=DropTargetId::new("zone-b")
                                    class="p-4 h-full"
                                    hover_class="bg-green-50"
                                >
                                    <h3 class="font-semibold mb-4 text-green-900">"Zone B"</h3>
                                    
                                    <SortableList
                                        items=zone_b
                                        on_reorder=on_reorder_b
                                        item_id=|task: &Task| task.id.clone()
                                        render=move |task: Task| {
                                            view! {
                                                <div class=format!("p-3 rounded-lg shadow-sm cursor-grab active:cursor-grabbing {}", task.color)>
                                                    <span class="font-medium">{task.label}</span>
                                                </div>
                                            }
                                        }
                                    />
                                </DropZone>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div class="border rounded-lg p-6">
                            <h3 class="font-semibold mb-4">"Event Log"</h3>
                            <div class="space-y-2 text-sm font-mono max-h-[300px] overflow-y-auto">
                                {move || log.get().into_iter().map(|msg| {
                                    view! { <div class="text-xs">{msg}</div> }
                                }).collect_view()}
                            </div>
                        </div>

                        <div class="p-4 bg-green-50 border border-green-200 rounded">
                            <p class="text-sm font-semibold text-green-900">"✅ Features"</p>
                            <ul class="text-xs text-green-700 mt-2 space-y-1">
                                <li>"• Visual drop indicators"</li>
                                <li>"• Reorder within zone"</li>
                                <li>"• Move between zones"</li>
                                <li>"• Zone hover feedback"</li>
                                <li>"• Event logging"</li>
                            </ul>
                        </div>

                        <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                            <p class="text-sm font-semibold text-blue-900">"🎯 Use Cases"</p>
                            <ul class="text-xs text-blue-700 mt-2 space-y-1">
                                <li>"• Kanban boards"</li>
                                <li>"• Task management"</li>
                                <li>"• Workflow steps"</li>
                                <li>"• File organization"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
    }
}
