use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableHeadPrimitive, DataTableHeadRowPrimitive, DataTableHeadCellPrimitive,
    DragHandlePrimitive, ResizeHandlePrimitive,
};
use super::types::ColumnDef;
use super::PinPosition;

#[component]
pub fn DataTableHeader<T: Clone + PartialEq + Send + Sync + 'static>(
    columns: Signal<Vec<ColumnDef<T>>>,
    sort_column: RwSignal<Option<String>>,
    sort_ascending: RwSignal<bool>,
    current_page: RwSignal<usize>,
    #[prop(into)] draggable: MaybeSignal<bool>,
    #[prop(into, optional)] resizable: Option<MaybeSignal<bool>>,
    #[prop(default = RwSignal::new(std::collections::HashMap::new()))] column_widths: RwSignal<std::collections::HashMap<String, u32>>,
    #[prop(into, default = "datatable".to_string())] table_id: String,
    #[prop(into, optional)] pinnable: Option<MaybeSignal<bool>>,
    #[prop(optional)] pinned_columns: Option<RwSignal<std::collections::HashMap<String, PinPosition>>>,
    #[prop(optional)] pin_offsets: Option<Signal<std::collections::HashMap<String, u32>>>,
    // "center" | "left" | "right" — define ícone e estado inicial do pin button
    #[prop(default = "center")] panel: &'static str,
) -> impl IntoView {
    let resizable_signal = resizable.unwrap_or_else(|| MaybeSignal::Static(false));
    let pinnable_signal  = pinnable.unwrap_or_else(|| MaybeSignal::Static(false));

    let handle_sort = move |col: String| {
        if sort_column.get().as_ref() == Some(&col) {
            sort_ascending.update(|asc| *asc = !*asc);
        } else {
            sort_column.set(Some(col));
            sort_ascending.set(true);
        }
        current_page.set(1);
    };

    // ícone e estado do pin baseado no painel
    let (pin_icon, pin_state) = match panel {
        "left"  => ("⬅📌", "pinned-left"),
        "right" => ("📌➡", "pinned-right"),
        _       => ("📍",   "unpinned"),
    };

    view! {
        <DataTableHeadPrimitive
            id=format!("{}-resize", table_id)
            attr:data-resize-container=""
            attr:data-pin-container=""
            attr:data-drag-container=move || if draggable.get() { Some("") } else { None }
        >
            <DataTableHeadRowPrimitive>
                {move || {
                    columns.with(|cols| {
                        cols.iter().enumerate().map(|(idx, col)| {
                            let col_id           = col.id.clone();
                            let col_label        = col.label.clone();
                            let col_sortable     = col.sortable;
                            let col_id_sort      = col.id.clone();
                            let col_id_indicator = col.id.clone();
                            let col_id_resize    = col.id.clone();
                            let col_id_pin       = col.id.clone();
                            let drag_id_val      = col.id.clone();
                            let drag_index_val   = idx.to_string();

                            view! {
                                <DataTableHeadCellPrimitive
                                    sort_key=if col_sortable { col_id.clone() } else { String::new() }
                                    attr:data-drag-item=move || if draggable.get() { Some("") } else { None }
                                    attr:data-drag-id=move || if draggable.get() { Some(drag_id_val.clone()) } else { None }
                                    attr:data-drag-index=move || if draggable.get() { Some(drag_index_val.clone()) } else { None }
                                >
                                    <div style="display: flex; align-items: center; gap: 0.5rem; position: relative;">
                                        <Show when=move || draggable.get()>
                                            <DragHandlePrimitive class="cursor-move".to_string()>
                                                "⋮⋮"
                                            </DragHandlePrimitive>
                                        </Show>
                                        {if col_sortable {
                                            view! {
                                                <button
                                                    on:click=move |_| handle_sort(col_id_sort.clone())
                                                    class="flex-1 text-left flex items-center gap-2"
                                                    style="background:none;border:none;cursor:pointer;padding:0;"
                                                >
                                                    {col_label.clone()}
                                                    <span>
                                                        {move || {
                                                            if sort_column.get().as_ref() == Some(&col_id_indicator) {
                                                                if sort_ascending.get() { "▲" } else { "▼" }
                                                            } else { "⇅" }
                                                        }}
                                                    </span>
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span class="flex-1">{col_label}</span> }.into_any()
                                        }}
                                        <Show when=move || pinnable_signal.get()>
                                            <button
                                                type="button"
                                                data-pin-button=""
                                                data-pin-col-id=col_id_pin.clone()
                                                data-pin-state=pin_state
                                                title="Pin/Unpin column"
                                                style="background:none;border:none;padding:4px;cursor:pointer;font-size:0.75rem;flex-shrink:0;"
                                            >
                                                {pin_icon}
                                            </button>
                                        </Show>
                                        <Show when=move || resizable_signal.get()>
                                            <ResizeHandlePrimitive col_id=col_id_resize.clone() />
                                        </Show>
                                    </div>
                                </DataTableHeadCellPrimitive>
                            }
                        }).collect_view()
                    })
                }}
            </DataTableHeadRowPrimitive>
        </DataTableHeadPrimitive>
    }
}
