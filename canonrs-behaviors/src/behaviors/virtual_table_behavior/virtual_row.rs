use leptos::prelude::*;
use super::types::{VirtualRow, VirtualColumn};

#[component]
pub fn VirtualRowComponent(
    row: VirtualRow,
    columns: Vec<VirtualColumn>,
    row_height: f64,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-row=""
            style={format!("--row-height: {}px;", row_height)}
        >
            {columns.iter().enumerate().map(|(i, col)| {
                let width_style = if let Some(width) = col.width {
                    format!("--col-width: {}px;", width)
                } else if let Some(flex) = col.flex {
                    format!("--col-flex: {};", flex)
                } else {
                    "--col-flex: 1;".to_string()
                };
                
                let value = row.data.get(i).cloned().unwrap_or_default();
                
                view! {
                    <div
                        attr:data-virtual-cell=""
                        attr:data-align={col.align.as_str()}
                        style={width_style}
                    >
                        {value}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
