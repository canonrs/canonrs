use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DataTableDensity {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

impl DataTableDensity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }
}

#[component]
pub fn DataTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = DataTableDensity::Comfortable)] density: DataTableDensity,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] page_size: String,
    #[prop(default = String::new())] current_page: String,
    #[prop(default = String::new())] total_pages: String,
    #[prop(default = String::new())] sync_chart: String,
) -> impl IntoView {
    view! {
        <div
            data-datatable=""
            data-density={density.as_str()}
            data-page-size=page_size
            data-current-page=current_page
            data-total-pages=total_pages
            class=class
            id=id
            data-table-sync-chart=sync_chart
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableToolbarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-toolbar="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableScrollPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-scroll="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <table data-datatable-table="" class=class>
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn DataTableHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <thead data-datatable-head="" data-resize-container="" class=class id=id>
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn DataTableHeadRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-datatable-head-row="" class=class>
            {children.map(|c| c())}
        </tr>
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

impl SortDirection {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Ascending => Some("ascending"),
            Self::Descending => Some("descending"),
            Self::None => None,
        }
    }
}

#[component]
pub fn DataTableHeadCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] sort_key: String,
    #[prop(default = SortDirection::None)] sort_direction: SortDirection,
    #[prop(default = String::new())] col_index: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <th
            data-datatable-head-cell=""
            scope="col"
            aria-sort="none"
            data-sort-key=sort_key
            data-col-index=col_index
            style={(!style.is_empty()).then(|| style.clone())}
            class=class
        >
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn DataTableBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-datatable-body="" class=class>
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn DataTableRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] row_id: String,
    #[prop(default = false)] selected: bool,
    #[prop(optional)] row_index: Option<usize>,
) -> impl IntoView {
    let id = if !row_id.is_empty() && !row_id.starts_with("row-") {
        format!("row-{}", row_id)
    } else {
        row_id.clone()
    };

    view! {
        <tr
            data-datatable-row=""
            data-row-id={(!row_id.is_empty()).then(|| row_id)}
            data-state={selected.then(|| "selected")}
            aria-rowindex={row_index.map(|i| (i + 1).to_string())}
            data-row-index={row_index.map(|i| i.to_string())}
            id={(!id.is_empty()).then(|| id)}
            class=class
        >
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn DataTableCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] col_index: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <td
            data-datatable-cell=""
            data-col-index=col_index
            class=class
            style={(!style.is_empty()).then(|| style.clone())}
        >
            {children.map(|c| c())}
        </td>
    }
}

#[component]
pub fn DataTableFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tfoot data-datatable-footer="" class=class>
            {children.map(|c| c())}
        </tfoot>
    }
}

#[component]
pub fn DataTablePaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-pagination="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableEmptyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-empty="" data-empty="true" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableLoadingPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-loading="" data-loading="true" class=class>
            {children.map(|c| c())}
        </div>
    }
}
