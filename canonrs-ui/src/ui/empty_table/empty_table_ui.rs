//! EmptyTable UI Component
//! Empty state for tables (Family E: Feedback)

use leptos::prelude::*;
use crate::primitives::{
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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyTablePrimitive colspan=colspan class=class id=id>
            <EmptyTableTitlePrimitive>{title}</EmptyTableTitlePrimitive>
            <EmptyTableDescriptionPrimitive>{description}</EmptyTableDescriptionPrimitive>
            {children.map(|c| c())}
        </EmptyTablePrimitive>
    }
}
