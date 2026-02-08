use leptos::prelude::*;
use super::empty_table_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <EmptyTable>"No data available"</EmptyTable>
    }
}
