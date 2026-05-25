//! @canon-level: strict
//! DataTable Island — Canon Rule #340 (zero-logic passthrough)

use leptos::prelude::*;
pub use canonrs_core::primitives::DataTableDensity;
pub use super::data_table_ui::{
    DataTableColumn,
    RowAction,
    BulkAction
};
use super::data_table_ui::DataTableStatic;
use super::types::{ExpandRenderFn, RowIdFn, RowLabelFn};

#[component]
pub fn DataTable<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 10)] page_size: usize,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] show_density: bool,
    #[prop(default = false)] resizable: bool,
    #[prop(default = vec![])] row_actions: Vec<RowAction>,
    #[prop(default = vec![])] bulk_actions: Vec<BulkAction>,
    #[prop(optional)] expand_render: Option<std::sync::Arc<dyn Fn(&T) -> leptos::prelude::AnyView + Send + Sync>>,
    #[prop(optional)] row_id_fn: Option<std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>>,
    #[prop(optional)] row_label_fn: Option<std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    match (expand_render, row_id_fn, row_label_fn) {
        (Some(er), Some(ri), Some(rl)) => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
                expand_render=ExpandRenderFn(Some(er)) row_id_fn=RowIdFn(Some(ri)) row_label_fn=RowLabelFn(Some(rl))
            />
        }.into_any(),
        (Some(er), None, None) => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
                expand_render=ExpandRenderFn(Some(er))
            />
        }.into_any(),
        (Some(er), Some(ri), None) => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
                expand_render=ExpandRenderFn(Some(er)) row_id_fn=RowIdFn(Some(ri))
            />
        }.into_any(),
        (None, Some(ri), Some(rl)) => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
                row_id_fn=RowIdFn(Some(ri)) row_label_fn=RowLabelFn(Some(rl))
            />
        }.into_any(),
        (None, Some(ri), None) => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
                row_id_fn=RowIdFn(Some(ri))
            />
        }.into_any(),
        _ => view! {
            <DataTableStatic
                data=data columns=columns density=density class=class
                page_size=page_size selectable=selectable show_density=show_density resizable=resizable
                row_actions=row_actions bulk_actions=bulk_actions
            />
        }.into_any(),
    }
}
