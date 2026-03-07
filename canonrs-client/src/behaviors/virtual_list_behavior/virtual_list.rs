use leptos::prelude::*;
use crate::primitives::virtual_list::{
    VirtualList as VirtualListPrimitive,
    VirtualListViewport as VirtualListViewportPrimitive,
    VirtualListContent as VirtualListContentPrimitive,
};
use crate::primitives::separator::SeparatorPrimitive;
use super::types::{ScrollState, VirtualListConfig};
use super::viewport::{calculate_visible_range, calculate_total_height, calculate_offset};
use wasm_bindgen::JsCast;

#[component]
pub fn VirtualList<T: Clone + PartialEq + Send + Sync + 'static>(
    items: Signal<Vec<T>>,
    item_renderer: ChildrenFn,
    #[prop(default = VirtualListConfig::default())] config: VirtualListConfig,
    #[prop(default = false)] show_separators: bool,
) -> impl IntoView {
    let scroll_state = RwSignal::new(ScrollState { scroll_top: 0.0 });

    let total_items = Memo::new(move |_| items.get().len());

    let visible_range = Memo::new(move |_| {
        calculate_visible_range(
            scroll_state.get(),
            total_items.get(),
            config,
        )
    });

    let total_height = Memo::new(move |_| {
        calculate_total_height(total_items.get(), config.item_height)
    });

    let offset = Memo::new(move |_| {
        calculate_offset(visible_range.get(), config.item_height)
    });

    view! {
        <VirtualListPrimitive
            attr:data-item-height={config.item_height}
            attr:data-viewport-height={config.viewport_height}
            attr:data-overscan={config.overscan}
            attr:data-total-items={total_items.get()}
            attr:data-total-height={total_height.get()}
            attr:aria-label="Virtual scrolling list"
        >
            <VirtualListViewportPrimitive
                on:scroll=move |ev| {
                    if let Some(target) = ev.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                        let scroll_top = target.scroll_top() as f64;
                        scroll_state.set(ScrollState { scroll_top });
                    }
                }
            >
                <VirtualListContentPrimitive
                    attr:data-offset={offset.get()}
                    attr:style={format!("transform: translateY({}px);", offset.get())}
                >
                    {item_renderer()}

                    {show_separators.then(|| view! {
                        <SeparatorPrimitive
                            orientation={"horizontal".to_string()}
                            decorative={true}
                        />
                    })}
                </VirtualListContentPrimitive>
            </VirtualListViewportPrimitive>
        </VirtualListPrimitive>
    }
}
