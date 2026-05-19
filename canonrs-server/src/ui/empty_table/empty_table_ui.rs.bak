#![allow(unreachable_pub, dead_code)]

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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyTablePrimitive colspan=colspan class=class>
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
