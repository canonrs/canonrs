use leptos::prelude::*;
use crate::ui::data_table::{DataTable, DataTableColumn};

#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonRow {
    pub aspect: String,
    pub traditional: String,
    pub canon: String,
}

#[component]
pub fn ComparisonBlock(
    rows: Vec<ComparisonRow>,
) -> impl IntoView {
    let columns = vec![
        DataTableColumn::new("aspect", "Aspect")
            .render(|row: &ComparisonRow| view! { <strong>{row.aspect.clone()}</strong> }),
        DataTableColumn::new("traditional", "Traditional")
            .render(|row: &ComparisonRow| view! { {row.traditional.clone()} }),
        DataTableColumn::new("canon", "Canon")
            .render(|row: &ComparisonRow| view! { {row.canon.clone()} }),
    ];

    view! {
        <section class="canon-comparison" attr:data-comparison="">
            <h3 class="canon-comparison-title">
                "Traditional vs Canon"
            </h3>

            <DataTable
                data=rows
                columns=columns
            />
        </section>
    }
}
