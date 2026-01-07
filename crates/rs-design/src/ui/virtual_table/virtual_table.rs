use leptos::prelude::*;
use super::types::{VirtualRow, VirtualColumn};
use super::viewport::VirtualViewport;
use super::header::VirtualTableHeader;
use super::empty_state::EmptyState;
use super::loading_skeleton::LoadingSkeleton;

#[component]
pub fn VirtualTable(
    #[prop(into)] rows: Signal<Vec<VirtualRow>>,
    columns: Vec<VirtualColumn>,
    #[prop(default = 36.0)] row_height: f64,
    #[prop(default = 600.0)] viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
    #[prop(default = true)] sticky_header: bool,
    #[prop(into, optional)] loading: Option<Signal<bool>>,
    #[prop(into, optional)] empty_message: Option<String>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let is_loading = move || {
        loading.map(|l| l.get()).unwrap_or(false)
    };
    
    let empty_msg = empty_message.unwrap_or_else(|| "No data available".to_string());
    
    view! {
        <div class=format!("virtual-table-container border border-border rounded-md overflow-hidden {}", class)>
            // Sticky header
            <VirtualTableHeader 
                columns=columns.clone()
                sticky=sticky_header
            />
            
            // Conteúdo
            <div class="relative">
                {move || {
                    if is_loading() {
                        view! {
                            <LoadingSkeleton 
                                rows=10
                                row_height=row_height
                            />
                        }.into_any()
                    } else if rows.get().is_empty() {
                        view! {
                            <EmptyState message=empty_msg.clone() />
                        }.into_any()
                    } else {
                        view! {
                            <VirtualViewport
                                rows=rows
                                columns=columns.clone()
                                row_height=row_height
                                viewport_height=viewport_height
                                overscan=overscan
                            />
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
