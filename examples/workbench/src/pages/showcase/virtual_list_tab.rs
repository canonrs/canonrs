use leptos::prelude::*;
use rs_design::ui::virtual_list::{VirtualList, VirtualListConfig};

#[derive(Clone, Debug, PartialEq)]
struct DemoItem {
    id: usize,
    label: String,
    category: String,
}

#[component]
pub fn VirtualListTab() -> impl IntoView {
    // Generate 50,000 items
    let (items, _set_items) = signal(
        (0..50000)
            .map(|i| DemoItem {
                id: i,
                label: format!("Item #{}", i),
                category: match i % 5 {
                    0 => "Critical",
                    1 => "High",
                    2 => "Medium",
                    3 => "Low",
                    _ => "Info",
                }.to_string(),
            })
            .collect::<Vec<_>>()
    );

    let config = VirtualListConfig {
        item_height: 48.0,
        viewport_height: 600.0,
        overscan: 5,
    };

    let render_item = move |item: DemoItem, index: usize| {
        let badge_color = match item.category.as_str() {
            "Critical" => "bg-red-100 text-red-800",
            "High" => "bg-orange-100 text-orange-800",
            "Medium" => "bg-yellow-100 text-yellow-800",
            "Low" => "bg-blue-100 text-blue-800",
            _ => "bg-gray-100 text-gray-800",
        };

        view! {
            <div class="flex items-center justify-between px-4 py-3 border-b hover:bg-muted/50 transition-colors">
                <div class="flex items-center gap-3">
                    <span class="text-sm text-muted-foreground font-mono">
                        {format!("#{:05}", index)}
                    </span>
                    <span class="text-sm font-medium">{item.label}</span>
                </div>
                <span class=format!("text-xs px-2 py-1 rounded {}", badge_color)>
                    {item.category}
                </span>
            </div>
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Virtual List (Generic)"</h2>
                <p class="text-muted-foreground">"Handles 50,000+ items with smooth 60fps scrolling"</p>
            </div>

            <div class="grid grid-cols-3 gap-6">
                <div class="col-span-2">
                    <div class="border rounded-lg overflow-hidden">
                        <div class="p-3 bg-muted border-b">
                            <h3 class="font-semibold text-sm">"Virtual List"</h3>
                        </div>
                        <VirtualList
                            items=items
                            config=config
                            children=render_item
                        />
                    </div>
                </div>

                <div class="space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Performance Stats"</h3>
                        <div class="space-y-3 text-sm">
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Total Items:"</span>
                                <code>{move || items.get().len()}</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Item Height:"</span>
                                <code>"48px"</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Viewport:"</span>
                                <code>"600px"</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Overscan:"</span>
                                <code>"5 rows"</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Rendered:"</span>
                                <code>"~25 items"</code>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 bg-green-50 border border-green-200 rounded">
                        <p class="text-sm font-semibold text-green-900">"⚡ Generic VirtualList"</p>
                        <ul class="text-xs text-green-700 mt-2 space-y-1">
                            <li>"• Works with any type T"</li>
                            <li>"• Custom render function"</li>
                            <li>"• No DOM nodes for hidden items"</li>
                            <li>"• 60fps smooth scrolling"</li>
                            <li>"• Low memory footprint"</li>
                        </ul>
                    </div>

                    <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                        <p class="text-sm font-semibold text-blue-900">"🧠 Use Cases"</p>
                        <ul class="text-xs text-blue-700 mt-2 space-y-1">
                            <li>"• Audit logs (millions)"</li>
                            <li>"• Command palette search"</li>
                            <li>"• Activity feeds"</li>
                            <li>"• Event streams"</li>
                            <li>"• Large datasets"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
