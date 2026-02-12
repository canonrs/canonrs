use leptos::prelude::*;
use super::{
    DataTable, DataTableToolbar, DataTableScroll, DataTableTable,
    DataTableHead, DataTableHeadRow, DataTableHeadCell,
    DataTableBody, DataTableRow, DataTableCell,
    DataTablePagination, DataTableEmpty
};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <DataTable>
            <DataTableToolbar>
                <div>"Search..."</div>
                <div>"Filters"</div>
            </DataTableToolbar>

            <DataTableScroll>
                <DataTableTable>
                    <DataTableHead>
                        <DataTableHeadRow>
                            <DataTableHeadCell sort_key="name">
                                "Name"
                            </DataTableHeadCell>
                            <DataTableHeadCell sort_key="email">
                                "Email"
                            </DataTableHeadCell>
                            <DataTableHeadCell>"Status"</DataTableHeadCell>
                        </DataTableHeadRow>
                    </DataTableHead>

                    <DataTableBody>
                        <DataTableRow id="1".to_string()>
                            <DataTableCell>"Alice"</DataTableCell>
                            <DataTableCell>"alice@example.com"</DataTableCell>
                            <DataTableCell>"Active"</DataTableCell>
                        </DataTableRow>
                        <DataTableRow id="2".to_string()>
                            <DataTableCell>"Bob"</DataTableCell>
                            <DataTableCell>"bob@example.com"</DataTableCell>
                            <DataTableCell>"Inactive"</DataTableCell>
                        </DataTableRow>
                    </DataTableBody>
                </DataTableTable>
            </DataTableScroll>

            <DataTablePagination>
                <div>"Showing 1-2 of 2"</div>
                <div>"Prev | Next"</div>
            </DataTablePagination>
        </DataTable>
    }
}
