use leptos::prelude::*;
use super::selection_context::use_selection;

/// SelectableItem - Item that supports multi-selection
///
/// **Type:** Stateful Component (Type 2)
/// **Keyboard:** Click, Ctrl+Click, Shift+Click
#[component]
pub fn SelectableItem<T>(
    /// Unique item identifier
    item: T,
    
    /// Children (item content)
    children: Children,
    
    /// Additional CSS classes
    #[prop(default = String::new(), into)]
    class: String,
    
    /// Selected state CSS classes
    #[prop(default = "bg-primary/10 border-l-2 border-primary".to_string(), into)]
    selected_class: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    let context = use_selection::<T>();
    
    let item_clone = item.clone();
    let is_selected = context.is_selected(&item_clone);
    
    let on_click = move |ev: web_sys::MouseEvent| {
        let item = item.clone();
        
        #[cfg(target_arch = "wasm32")]
        {
            if ev.ctrl_key() || ev.meta_key() {
                // Ctrl+Click: Toggle selection
                context.toggle(item);
            } else if ev.shift_key() {
                // Shift+Click: Range selection (TODO: implement range)
                context.add(item);
            } else {
                // Click: Single selection
                context.select_single(item);
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // SSR: always single select
            context.select_single(item);
        }
    };
    
    view! {
        <div
            class=move || {
                let base = format!("selectable-item cursor-pointer transition-colors {}", class);
                if is_selected.get() {
                    format!("{} {}", base, selected_class)
                } else {
                    base
                }
            }
            on:click=on_click
        >
            {children()}
        </div>
    }
}
