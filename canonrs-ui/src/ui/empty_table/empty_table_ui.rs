//! EmptyTable UI Component
//! Padronização de estados vazios em tabelas

use leptos::prelude::*;
use crate::primitives::empty_state::EmptyStatePrimitive;
use crate::primitives::table::TablePrimitive;

#[component]
pub fn EmptyTable(
    #[prop(default = "No data available".to_string())] message: String,
    #[prop(default = "Add your first item to get started".to_string())] description: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 999)] colspan: i32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TablePrimitive
            class={class}
            id={id}
        >
            <tbody>
                <tr>
                    <td colspan={colspan} class="text-center py-12">
                        <EmptyStatePrimitive>
                            <div class="flex flex-col items-center gap-3">
                                <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-muted-foreground">
                                    <rect width="20" height="14" x="2" y="7" rx="2" ry="2"/>
                                    <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/>
                                </svg>
                                <div class="space-y-1 text-center">
                                    <h3 class="font-semibold text-lg">{message}</h3>
                                    <p class="text-sm text-muted-foreground">{description}</p>
                                </div>
                                {children.map(|c| c())}
                            </div>
                        </EmptyStatePrimitive>
                    </td>
                </tr>
            </tbody>
        </TablePrimitive>
    }
}
