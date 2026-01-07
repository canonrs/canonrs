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
            class="virtual-row flex border-b border-border"
            style=format!("height: {}px;", row_height)
        >
            {columns.iter().enumerate().map(|(i, col)| {
                let width_style = if let Some(width) = col.width {
                    format!("width: {}px; flex-shrink: 0;", width)
                } else if let Some(flex) = col.flex {
                    format!("flex: {};", flex)
                } else {
                    "flex: 1;".to_string()
                };
                
                let align_class = match col.align {
                    super::types::ColumnAlign::Left => "text-left",
                    super::types::ColumnAlign::Center => "text-center",
                    super::types::ColumnAlign::Right => "text-right",
                };
                
                let value = row.data.get(i).cloned().unwrap_or_default();
                
                view! {
                    <div 
                        class=format!("virtual-cell px-[0.5rem] flex items-center font-mono text-sm {}", align_class)
                        style=width_style
                    >
                        {value}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
