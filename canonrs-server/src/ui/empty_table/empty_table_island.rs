use leptos::prelude::*;
use super::empty_table_ui::EmptyTable;

#[island]
pub fn EmptyTableIsland(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] colspan: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let title       = title.unwrap_or_else(|| "No data available".to_string());
    let description = description.unwrap_or_else(|| "Add your first item to get started".to_string());
    let colspan     = colspan.unwrap_or(999);
    let class       = class.unwrap_or_default();

    view! {
        <EmptyTable
            title=title
            description=description
            colspan=colspan
            class=class
        />
    }
}
