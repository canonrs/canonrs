use leptos::prelude::*;
use super::types::{VirtualColumn, ColumnAlign};

#[component]
pub fn VirtualTableHeader(
    columns: Vec<VirtualColumn>,
    #[prop(default = true)] sticky: bool,
) -> impl IntoView {
    let sticky_class = if sticky {
        "sticky top-0 z-10 bg-surface border-b-2 border-border"
    } else {
        "border-b-2 border-border"
    };

    view! {
        <div class=format!("virtual-header flex {}", sticky_class)>
            {columns.iter().map(|col| {
                let width_style = if let Some(width) = col.width {
                    format!("width: {}px; flex-shrink: 0;", width)
                } else if let Some(flex) = col.flex {
                    format!("flex: {};", flex)
                } else {
                    "flex: 1;".to_string()
                };
                
                let align_class = match col.align {
                    ColumnAlign::Left => "justify-start",
                    ColumnAlign::Center => "justify-center",
                    ColumnAlign::Right => "justify-end",
                };
                
                view! {
                    <div 
                        class=format!("virtual-header-cell px-[0.5rem] py-[0.75rem] flex items-center font-semibold text-sm {}", align_class)
                        style=width_style
                    >
                        {col.key.clone()}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
