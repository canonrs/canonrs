//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed column widths for table layout
//! @canon-owner: data-team
//! @canon-target-date: 2025-03-15

use crate::blocks::data_table_row_actions::{DataTableRowActions, RowAction, RowActionVariant};
use crate::ui::button::{ButtonVariant, ButtonType};
use leptos::prelude::*;
use crate::ui::*;
use crate::ui::table::*;
use std::sync::Arc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
pub enum SortDirection {
    None,
    Asc,
    Desc,
}

impl SortDirection {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Asc,
            Self::Asc => Self::Desc,
            Self::Desc => Self::None,
        }
    }
}

pub struct DataTableColumn<T>
where
    T: Clone + Send + Sync + PartialEq + 'static,
{
    pub id: String,
    pub header: String,
    pub render: Arc<dyn Fn(&T) -> AnyView + Send + Sync>,
    pub class: String,
    pub can_hide: bool,
    pub sortable: bool,
    pub sort_fn: Option<Arc<dyn Fn(&T, &T) -> Ordering + Send + Sync>>,
}

impl<T: Clone + Send + Sync + PartialEq + 'static> DataTableColumn<T> {
    pub fn new(
        id: impl Into<String>,
        header: impl Into<String>,
        render: impl Fn(&T) -> AnyView + Send + Sync + 'static,
    ) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            header: header.into(),
            render: Arc::new(render),
            class: String::new(),
            can_hide: true,
            sortable: false,
            sort_fn: None,
        }
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }

    pub fn with_can_hide(mut self, can_hide: bool) -> Self {
        self.can_hide = can_hide;
        self
    }

    pub fn with_sort(
        mut self,
        sort_fn: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static,
    ) -> Self {
        self.sortable = true;
        self.sort_fn = Some(Arc::new(sort_fn));
        self
    }
}

impl<T: Clone + Send + Sync + PartialEq + 'static> Clone for DataTableColumn<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            header: self.header.clone(),
            render: Arc::clone(&self.render),
            class: self.class.clone(),
            can_hide: self.can_hide,
            sortable: self.sortable,
            sort_fn: self.sort_fn.as_ref().map(Arc::clone),
        }
    }
}

#[component]
fn DataTableRow<T>(
    item: T,
    row_id: String,
    columns: Vec<DataTableColumn<T>>,
    visible_columns: Memo<HashMap<String, bool>>,
    selectable: bool,
    selected: RwSignal<HashSet<String>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + PartialEq + 'static,
{
    let row_id = StoredValue::new(row_id);
    let columns = StoredValue::new(columns);

    let is_selected = Memo::new(move |_| {
        selected.get().contains(&row_id.get_value())
    });

    let handle_select = Callback::new(move |checked: bool| {
        let id = row_id.get_value();
        selected.update(|set| {
            if checked {
                set.insert(id);
            } else {
                set.remove(&id);
            }
        });
    });

    view! {
        <TableRowSelectable selected=Signal::derive(move || is_selected.get())>
            {selectable.then(|| view! {
                <TableCell class="w-[50px]">
                    <Checkbox
                        checked=Signal::derive(move || is_selected.get())
                        on_checked_change=handle_select
                    />
                </TableCell>
            })}

            <For
                each=move || {
                    let visible = visible_columns.get();
                    columns.get_value()
                        .into_iter()
                        .filter(|col| {
                            visible.get(&col.id).copied().unwrap_or(true)
                        })
                        .collect::<Vec<_>>()
                }
                key=|col| col.id.clone()
                children=move |col| {
                    let content = (col.render)(&item);
                    let class = col.class.clone();
                    view! {
                        <TableCell class=class>
                            {content}
                        </TableCell>
                    }
                }
            />
        </TableRowSelectable>
    }
}

#[component]
pub fn DataTable<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(optional)] get_id: Option<Arc<dyn Fn(&T) -> String + Send + Sync>>,
    #[prop(optional, into)] search_placeholder: String,
    #[prop(optional)] filter_fn: Option<Arc<dyn Fn(&T, &str) -> bool + Send + Sync>>,
    #[prop(default = false)] selectable: bool,
    #[prop(default = true)] show_view_options: bool,
) -> impl IntoView
where
    T: Clone + Send + Sync + PartialEq + 'static,
{
    let columns = Arc::new(columns);
    let get_id = Arc::new(get_id.unwrap_or_else(|| Arc::new(|_| uuid::Uuid::new_v4().to_string())));

    let get_id_for_all = Arc::clone(&get_id);
    let get_id_for_select_all = Arc::clone(&get_id);
    let get_id_for_key = Arc::clone(&get_id);
    let get_id_for_children = Arc::clone(&get_id);

    let current_page = RwSignal::new(0_usize);
    let search_filter = RwSignal::new(String::new());
    let selected_rows = RwSignal::new(HashSet::<String>::new());
    
    // Sorting state
    let sort_column = RwSignal::new(Option::<String>::None);
    let sort_direction = RwSignal::new(SortDirection::None);

    // Column visibility
    let visible_columns_signal = RwSignal::new(HashMap::<String, bool>::new());
    let visible_columns = Memo::new(move |_| visible_columns_signal.get());

    let column_ids: Vec<String> = columns.iter()
        .filter(|col| col.can_hide)
        .map(|col| col.id.clone())
        .collect();

    let search_placeholder = if search_placeholder.is_empty() {
        "Search...".to_string()
    } else {
        search_placeholder
    };

    let filtered_data = Memo::new(move |_| {
        let filter = search_filter.get().to_lowercase();
        if filter.is_empty() {
            data.clone()
        } else {
            data.iter()
                .filter(|item| {
                    if let Some(ref f) = filter_fn {
                        f(item, &filter)
                    } else {
                        true
                    }
                })
                .cloned()
                .collect()
        }
    });

    let sort_columns = Arc::clone(&columns);
    let sorted_data = Memo::new(move |_| {
        let mut items = filtered_data.get();
        
        if let Some(col_id) = sort_column.get() {
            if let Some(column) = sort_columns.iter().find(|c| c.id == col_id) {
                if let Some(ref sort_fn) = column.sort_fn {
                    let direction = sort_direction.get();
                    items.sort_by(|a, b| {
                        let cmp = sort_fn(a, b);
                        match direction {
                            SortDirection::Asc => cmp,
                            SortDirection::Desc => cmp.reverse(),
                            SortDirection::None => Ordering::Equal,
                        }
                    });
                }
            }
        }
        
        items
    });

    let page_size = 10_usize;
    let total_pages = Memo::new(move |_| {
        let total = sorted_data.get().len();
        ((total + page_size - 1) / page_size).max(1)
    });

    let paginated_data = Memo::new(move |_| {
        let all_data = sorted_data.get();
        let page = current_page.get();
        let start = page * page_size;
        let end = (start + page_size).min(all_data.len());
        all_data[start..end].to_vec()
    });

    let all_selected = Signal::derive(move || {
        let page_data = paginated_data.get();
        let selected = selected_rows.get();
        !page_data.is_empty() && page_data.iter().all(|item| {
            selected.contains(&get_id_for_all(item))
        })
    });

    let header_columns = Arc::clone(&columns);
    let body_columns = Arc::clone(&columns);

    view! {
        <div class="space-y-4">
            <div class="flex items-center py-4 gap-2">
                <Input
                    placeholder=search_placeholder
                    value=search_filter
                    class="max-w-sm"
                />
                {show_view_options.then(|| view! {
                    <crate::blocks::DataTableViewOptions
                        columns=column_ids
                        visible_columns=visible_columns_signal
                    />
                })}
            </div>

            <div class="overflow-x-auto rounded-md border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            {selectable.then(|| view! {
                                <TableHead class="w-[50px]">
                                    <Checkbox
                                        checked=all_selected
                                        on_checked_change=Callback::new(move |checked: bool| {
                                            let page_data = paginated_data.get_untracked();
                                            selected_rows.update(|set| {
                                                if checked {
                                                    for item in page_data.iter() {
                                                        set.insert(get_id_for_select_all(item));
                                                    }
                                                } else {
                                                    for item in page_data.iter() {
                                                        set.remove(&get_id_for_select_all(item));
                                                    }
                                                }
                                            });
                                        })
                                    />
                                </TableHead>
                            })}

                            {move || {
                                let visible = visible_columns.get();
                                header_columns.iter()
                                    .filter(|col| {
                                        visible.get(&col.id).copied().unwrap_or(true)
                                    })
                                    .map(|col| {
                                        let header_text = col.header.clone();
                                        let class = col.class.clone();
                                        let sortable = col.sortable;
                                        let col_id = col.id.clone();
                                        
                                        if sortable {
                                            view! {
                                                <TableHead class=class>
                                                    <Button
                                                        variant=ButtonVariant::Ghost
                                                        class="h-8 px-2"
                                                        on:click=move |_| {
                                                            let current_col = sort_column.get_untracked();
                                                            let current_dir = sort_direction.get_untracked();
                                                            
                                                            if current_col.as_ref() == Some(&col_id) {
                                                                sort_direction.set(current_dir.next());
                                                            } else {
                                                                sort_column.set(Some(col_id.clone()));
                                                                sort_direction.set(SortDirection::Asc);
                                                            }
                                                        }
                                                    >
                                                        {header_text.clone()}
                                                        <svg
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            width="16"
                                                            height="16"
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            class="ml-2 h-4 w-4"
                                                        >
                                                            <path d="m21 16-4 4-4-4"/>
                                                            <path d="M17 20V4"/>
                                                            <path d="m3 8 4-4 4 4"/>
                                                            <path d="M7 4v16"/>
                                                        </svg>
                                                    </Button>
                                                </TableHead>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <TableHead class=class>
                                                    {header_text}
                                                </TableHead>
                                            }.into_any()
                                        }
                                    }).collect_view()
                            }}
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <For
                            each=move || paginated_data.get()
                            key=move |item| get_id_for_key(item)
                            children=move |item: T| {
                                let row_id = get_id_for_children(&item);
                                view! {
                                    <DataTableRow<T>
                                        item=item
                                        row_id=row_id
                                        columns=body_columns.as_ref().clone()
                                        visible_columns=visible_columns
                                        selectable=selectable
                                        selected=selected_rows
                                    />
                                }
                            }
                        />
                    </TableBody>
                </Table>
            </div>

            <div class="flex items-center justify-between py-4">
                <div class="flex-1 text-sm text-muted-foreground">
                    {move || {
                        let selected_count = selected_rows.get().len();
                        let total = filtered_data.get().len();
                        if selected_count > 0 {
                            format!("{} of {} row(s) selected.", selected_count, total)
                        } else {
                            format!("{} row(s).", total)
                        }
                    }}
                </div>
                <div class="flex items-center space-x-2">
                    <Button
                        variant=ButtonVariant::Outline
                        on:click=move |_| {
                            current_page.update(|p| if *p > 0 { *p -= 1 });
                        }
                    >
                        Previous
                    </Button>
                    <span class="text-sm">
                        Page {move || current_page.get() + 1} / {move || total_pages.get()}
                    </span>
                    <Button
                        variant=ButtonVariant::Outline
                        on:click=move |_| {
                            let max = total_pages.get_untracked();
                            current_page.update(|p| {
                                if *p + 1 < max {
                                    *p += 1;
                                }
                            });
                        }
                    >
                        Next
                    </Button>
                </div>
            </div>
        </div>
    }
}





// Helper: Create actions column
impl<T: Clone + Send + Sync + PartialEq + 'static> DataTableColumn<T> {
    pub fn actions(
        get_actions: impl Fn(&T) -> Vec<RowAction> + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: "actions".to_string(),
            header: "".to_string(),
            render: Arc::new(move |item: &T| -> AnyView {
                let actions = get_actions(item);
                view! {
                    <DataTableRowActions actions=actions />
                }.into_any()
            }),
            class: "w-[50px]".to_string(),
            can_hide: false,
            sortable: false,
            sort_fn: None,
        }
    }
}
