//! @canon-id: data-table
//! @canon-label: Data Table
//! @canon-family: data_display
//! @canon-category: Data
//! @canon-intent: Display sortable, filterable tabular data
//! @canon-description: Sortable data table component
//! @canon-composable: true
//! @canon-capabilities: Multiple
//! @canon-required-parts:
//! @canon-optional-parts: DataTablePagination, DataTableToolbar, DataTableColumn
//! @canon-tags: data-table, table, data, grid, sortable, filterable

use leptos::prelude::*;
use canonrs_core::primitives::{
    DataTablePrimitive, DataTableScrollPrimitive, DataTableTablePrimitive,
    DataTableHeadPrimitive, DataTableHeadRowPrimitive, DataTableHeadCellPrimitive,
    DataTableBodyPrimitive, DataTableRowPrimitive, DataTableCellPrimitive,
    DataTableDensity,
};

#[derive(Clone)]
pub struct DataTableColumn<T> {
    pub key: String,
    pub label: String,
    pub render: std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>,
}

impl<T> DataTableColumn<T> {
    pub fn new(key: impl Into<String>, label: impl Into<String>, render: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        Self { key: key.into(), label: label.into(), render: std::sync::Arc::new(render) }
    }
}

#[component]
pub fn DataTableCore<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let stored_columns = StoredValue::new(columns);

    view! {
        <DataTablePrimitive density=density class=class>
            <DataTableScrollPrimitive>
                <DataTableTablePrimitive>
                    <DataTableHeadPrimitive>
                        <DataTableHeadRowPrimitive>
                            {stored_columns.get_value().iter().map(|col| {
                                let key = col.key.clone();
                                let label = col.label.clone();
                                view! {
                                    <DataTableHeadCellPrimitive
                                        sort_key=key
                                        sort_direction=canonrs_core::primitives::SortDirection::None
                                    >
                                        {label}
                                    </DataTableHeadCellPrimitive>
                                }
                            }).collect::<Vec<_>>()}
                        </DataTableHeadRowPrimitive>
                    </DataTableHeadPrimitive>

                    <DataTableBodyPrimitive>
                        {data.into_iter().enumerate().map(|(idx, row)| {
                            let cols = stored_columns.get_value();
                            view! {
                                <DataTableRowPrimitive row_id=idx.to_string()>
                                    {cols.iter().map(|col| {
                                        let value = (col.render)(&row);
                                        view! { <DataTableCellPrimitive>{value}</DataTableCellPrimitive> }
                                    }).collect::<Vec<_>>()}
                                </DataTableRowPrimitive>
                            }
                        }).collect::<Vec<_>>()}
                    </DataTableBodyPrimitive>
                </DataTableTablePrimitive>
            </DataTableScrollPrimitive>
        </DataTablePrimitive>
    }
}
