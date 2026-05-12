//! DataTable Preview — dados mock + config_for + PreviewUnified

use leptos::prelude::*;
use super::data_table_boundary::DataTable;
use super::data_table_data::{DataTableConfig, mock_users, user_columns};
use super::data_table_ui::{BulkAction, RowAction};

#[component]
pub fn DataTableUnifiedBoundary(config: DataTableConfig) -> impl IntoView {
    let data    = mock_users();
    let columns = user_columns();
    view! {
        <DataTable
            data=data
            columns=columns
            page_size=config.page_size
            selectable=config.selectable
            show_density=config.show_density
            resizable=config.resizable
            density=config.density
            row_actions=config.row_actions
            bulk_actions=config.bulk_actions
        />
    }
}

pub fn config_for(demo: &str) -> DataTableConfig {
    match demo {
        "basic" => DataTableConfig { ..Default::default() },
        "tier1" => DataTableConfig {
            show_density: true,
            resizable:    true,
            selectable:   true,
            bulk_actions: vec![
                BulkAction::new("delete", "Delete").danger(),
                BulkAction::new("export", "Export"),
            ],
            row_actions: vec![
                RowAction::new("edit",   "Edit"),
                RowAction::new("delete", "Delete").danger(),
            ],
            ..Default::default()
        },
        _ => DataTableConfig::default(),
    }
}

#[component]
pub fn DataTablePreviewUnified(#[prop(into)] demo: String) -> impl IntoView {
    let config = config_for(&demo);
    view! { <DataTableUnifiedBoundary config=config /> }
}
