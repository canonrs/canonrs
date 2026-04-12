//! @canon-level: strict
//! EmptyTable Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::empty_table_ui::EmptyTable as EmptyTableUi;

#[component]
pub fn EmptyTable(
    #[prop(into, default = String::from("No data available"))] title: String,
    #[prop(into, default = String::from("Add your first item to get started"))] description: String,
    #[prop(default = 999u32)] colspan: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <EmptyTableUi title=title description=description colspan=colspan class=class />
};
}
