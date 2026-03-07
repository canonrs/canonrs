use leptos::prelude::*;
use super::types::{VirtualRow, VirtualColumn};
use super::viewport::VirtualViewport;
use super::header::VirtualTableHeader;
use super::empty_state::EmptyState;
use super::loading_skeleton::LoadingSkeleton;

#[component]
pub fn VirtualTable(
    rows: Signal<Vec<VirtualRow>>,
    columns: Vec<VirtualColumn>,
    #[prop(default = 36.0)] row_height: f64,
    #[prop(default = 600.0)] viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
    #[prop(default = true)] sticky_header: bool,
    #[prop(default = Signal::derive(|| false))] loading: Signal<bool>,
    #[prop(default = "No data available".to_string())] empty_message: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let is_loading = Memo::new(move |_| {
        loading.get()
    });

    let empty_msg = empty_message;

    view! {
        <div class={format!("virtual-table-container border border-border rounded-md overflow-hidden {}", class)}>
            <VirtualTableHeader
                columns={columns.clone()}
                sticky={sticky_header}
            />

            <div class="relative">
                {move || {
                    if is_loading.get() {
                        view! {
                            <LoadingSkeleton
                                rows={10}
                                row_height={row_height}
                            />
                        }.into_any()
                    } else if rows.get().is_empty() {
                        view! {
                            <EmptyState message={empty_msg.clone()} />
                        }.into_any()
                    } else {
                        view! {
                            <VirtualViewport
                                rows={rows}
                                columns={columns.clone()}
                                row_height={row_height}
                                viewport_height={viewport_height}
                                overscan={overscan}
                            />
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
