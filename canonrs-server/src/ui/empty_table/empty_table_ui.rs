//! @canon-level: ui
//! EmptyTable - Empty state for tables

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
    #[prop(default = 999)] colspan: i32,
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
                    colspan=3
                    title="No data available".to_string()
                    description="Add your first item to get started.".to_string()
                />
            </tbody>
        </table>
    }
}
