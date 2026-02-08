use leptos::prelude::*;
use crate::ui::table::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use crate::ui::checkbox::Checkbox;
use crate::ui::button::{Button, ButtonVariant};
use super::types::{DataTableColumn, SortDirection};
use std::collections::HashSet;
use std::sync::Arc;

#[component]
pub fn DataTable<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = false)] selectable: bool,
    #[prop(optional)] selected: Option<Signal<HashSet<String>>>,
    #[prop(optional)] _sort_column: Option<Signal<Option<String>>>,
    #[prop(optional)] _sort_direction: Option<Signal<SortDirection>>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let columns = Arc::new(columns);
    let data_len = data.len();

    let all_selected = Memo::new(move |_| {
        if let Some(sel) = selected {
            sel.get().len() == data_len && data_len > 0
        } else {
            false
        }
    });

    let columns_header = Arc::clone(&columns);
    let columns_body = Arc::clone(&columns);

    view! {
        <Table class={class}>
            <TableHeader>
                <TableRow>
                    {selectable.then(|| view! {
                        <TableHead class="w-12".to_string()>
                            <Checkbox checked=all_selected.get() />
                        </TableHead>
                    })}
                    {columns_header.iter().map(|col| {
                        let col_label = col.label.clone();
                        let col_sortable = col.sortable;
                        let col_width = col.width.clone().unwrap_or_default();

                        view! {
                            <TableHead class={col_width}>
                                {if col_sortable {
                                    view! {
                                        <Button
                                            variant=ButtonVariant::Ghost
                                            class="w-full justify-start".to_string()
                                        >
                                            {col_label.clone()}
                                            <span class="ml-2">"⇅"</span>
                                        </Button>
                                    }.into_any()
                                } else {
                                    view! { {col_label} }.into_any()
                                }}
                            </TableHead>
                        }
                    }).collect_view()}
                </TableRow>
            </TableHeader>
            <TableBody>
                {data.into_iter().enumerate().map(|(idx, item)| {
                    let row_id = idx.to_string();
                    let row_id_for_check = row_id.clone();
                    let cols = Arc::clone(&columns_body);

                    let is_selected = move || {
                        selected.map(|sel| sel.get().contains(&row_id_for_check)).unwrap_or(false)
                    };

                    view! {
                        <TableRow>
                            {selectable.then(|| view! {
                                <TableCell>
                                    <Checkbox checked=is_selected() />
                                </TableCell>
                            })}
                            {cols.iter().map(|col| {
                                let rendered = (col.render)(&item);
                                view! {
                                    <TableCell>
                                        {rendered}
                                    </TableCell>
                                }
                            }).collect_view()}
                        </TableRow>
                    }
                }).collect_view()}
            </TableBody>
        </Table>
    }
}
