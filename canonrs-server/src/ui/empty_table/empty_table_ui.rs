//! @canon-id: empty-table
//! @canon-label: Empty Table
//! @canon-family: utility
//! @canon-category: Display
//! @canon-intent: Show empty state inside a table
//! @canon-description: Empty table state display
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: empty-table, empty, no-records, no-data

use leptos::prelude::*;
use canonrs_core::primitives::{
    EmptyTablePrimitive,
    EmptyTableTitlePrimitive,
    EmptyTableDescriptionPrimitive,
};

#[component]
pub fn EmptyTable(
    #[prop(into, default = "No data available".to_string())] title: String,
    #[prop(into, default = "Add your first item to get started".to_string())] description: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 999)] colspan: u32,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <EmptyTablePrimitive colspan=colspan class={class.unwrap_or_default()}>
            <EmptyTableTitlePrimitive>{title}</EmptyTableTitlePrimitive>
            <EmptyTableDescriptionPrimitive>{description}</EmptyTableDescriptionPrimitive>
            {children.map(|c| c())}
        </EmptyTablePrimitive>
    }
}

#[component]
pub fn EmptyTablePreview() -> impl IntoView {
    view! {
        <table>
            <tbody>
                <EmptyTable
                    colspan=3u32
                    title="No data available".to_string()
                    description="Add your first item to get started.".to_string()
                />
            </tbody>
        </table>
    }
}
