//! EmptyTable UI Component
//! Padronização de estados vazios em tabelas

use leptos::prelude::*;

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
        <tr data-empty-table-row="" class=class id=id>
            <td colspan=colspan>
                <div data-empty-table-content="">
                    <div data-empty-table-title="">{title}</div>
                    <div data-empty-table-description="">{description}</div>
                    {children.map(|c| c())}
                </div>
            </td>
        </tr>
    }
}
