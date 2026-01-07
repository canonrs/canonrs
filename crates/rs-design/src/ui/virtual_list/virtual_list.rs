use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::types::{ScrollState, VirtualListConfig};
use super::viewport::{calculate_visible_range, calculate_total_height, calculate_offset};

/// VirtualList - High-performance list for large datasets
///
/// **Type:** Stateful Component (Type 2)
/// **Generic:** Works with any item type T
/// **Pattern:** Virtualized rendering (only visible items)
/// **Performance:** Handles 100,000+ items
#[component]
pub fn VirtualList<T, F, IV>(
    /// Items to render
    #[prop(into)]
    items: Signal<Vec<T>>,
    
    /// Render function for each item: (item: T, index: usize) -> View
    children: F,
    
    /// Configuration (item height, viewport height, overscan)
    #[prop(optional)]
    config: Option<VirtualListConfig>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    F: Fn(T, usize) -> IV + Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let config = config.unwrap_or_default();
    let scroll_state = RwSignal::new(ScrollState { scroll_top: 0.0 });

    let total_items = Signal::derive(move || items.get().len());

    let visible_range = Signal::derive(move || {
        calculate_visible_range(
            scroll_state.get(),
            total_items.get(),
            config,
        )
    });

    let total_height = Signal::derive(move || {
        calculate_total_height(total_items.get(), config.item_height)
    });

    let on_scroll = move |ev: web_sys::Event| {
        if let Some(target) = ev.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                scroll_state.update(|s| {
                    s.scroll_top = element.scroll_top() as f64;
                });
            }
        }
    };

    view! {
        <div
            class="virtual-list-viewport overflow-y-auto"
            style=format!("height: {}px", config.viewport_height)
            on:scroll=on_scroll
        >
            <div style=move || format!("height: {}px; position: relative", total_height.get())>
                <div style=move || {
                    let range = visible_range.get();
                    let offset = calculate_offset(range, config.item_height);
                    format!("position: absolute; top: {}px; left: 0; right: 0", offset)
                }>
                    {move || {
                        let range = visible_range.get();
                        let all_items = items.get();

                        all_items
                            .into_iter()
                            .enumerate()
                            .skip(range.start)
                            .take(range.end - range.start)
                            .map(|(index, item)| {
                                let render = children.clone();
                                view! {
                                    <div style=format!("height: {}px", config.item_height)>
                                        {render(item, index)}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
