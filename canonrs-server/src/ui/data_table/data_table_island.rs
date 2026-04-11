//! @canon-level: strict
//! DataTable Island — Canon Rule #340 (zero-logic passthrough)

use leptos::prelude::*;
use canonrs_core::primitives::DataTableDensity;
use super::data_table_ui::{DataTableStatic, DataTableColumn, RowAction};

#[component]
pub fn DataTableIsland<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 10)] page_size: usize,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] show_density: bool,
    #[prop(default = vec![])] row_actions: Vec<RowAction>,
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
        />
    }
}
