use leptos::prelude::*;
use super::types::{DataTableRequest, DataTableResponse, ColumnDef};

pub struct DataTableState<T: Clone + PartialEq + Send + Sync + 'static> {
    pub current_page: RwSignal<usize>,
    pub sort_column: RwSignal<Option<String>>,
    pub sort_ascending: RwSignal<bool>,
    pub filter_query: RwSignal<String>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub data: RwSignal<Vec<T>>,
    pub total: RwSignal<usize>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> DataTableState<T> {
    pub fn new<F>(
        columns: Signal<Vec<ColumnDef<T>>>,
        fetch_data: F,
        page_size: usize,
    ) -> Self
    where
        F: Fn(DataTableRequest) -> Result<DataTableResponse<T>, String> + 'static,
    {
        let current_page = RwSignal::new(1usize);
        let sort_column = RwSignal::new(None::<String>);
        let sort_ascending = RwSignal::new(true);
        let filter_query = RwSignal::new(String::new());
        let loading = RwSignal::new(false);
        let error = RwSignal::new(None::<String>);
        let data = RwSignal::new(Vec::<T>::new());
        let total = RwSignal::new(0usize);

        create_effect(move |_| {
            columns.track();
            current_page.track();
            sort_column.track();
            sort_ascending.track();
            filter_query.track();

            loading.set(true);
            error.set(None);

            let request = DataTableRequest {
                page: current_page.get(),
                page_size,
                sort_column: sort_column.get(),
                sort_ascending: sort_ascending.get(),
                filter_query: filter_query.get(),
            };

            match fetch_data(request) {
                Ok(response) => {
                    data.set(response.data);
                    total.set(response.total);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });

        Self { current_page, sort_column, sort_ascending, filter_query, loading, error, data, total }
    }
}
