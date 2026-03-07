use leptos::prelude::*;
use super::types::VirtualColumn;

#[component]
pub fn VirtualTableHeader(
    columns: Vec<VirtualColumn>,
    #[prop(default = true)] sticky: bool,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-header=""
            attr:data-sticky={sticky}
        >
            {columns.iter().map(|col| {
                let width_style = if let Some(width) = col.width {
                    format!("--col-width: {}px;", width)
                } else if let Some(flex) = col.flex {
                    format!("--col-flex: {};", flex)
                } else {
                    "--col-flex: 1;".to_string()
                };
                
                view! {
                    <div
                        attr:data-header-cell=""
                        attr:data-align={col.align.as_str()}
                        style={width_style}
                    >
                        {col.key.clone()}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
