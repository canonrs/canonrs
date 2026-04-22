//! @canon-level: strict
//! DataTable Island — Canon Rule #340 (zero-logic passthrough)

use leptos::prelude::*;
pub use canonrs_core::primitives::DataTableDensity;
use super::data_table_ui::{
    DataTableStatic,
    DataTableColumn,
    RowAction,
    BulkAction
};

#[component]
pub fn DataTable<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 10)] page_size: usize,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] show_density: bool,
    #[prop(default = vec![])] row_actions: Vec<RowAction>,
    #[prop(default = vec![])] bulk_actions: Vec<BulkAction>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    view! {
        <DataTableStatic
            data=data
            columns=columns
            density=density
            class=class
            page_size=page_size
            selectable=selectable
            show_density=show_density
            row_actions=row_actions
            bulk_actions=bulk_actions
        />
    }
}
