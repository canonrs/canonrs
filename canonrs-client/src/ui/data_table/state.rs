use leptos::prelude::*;
use super::types_pin::PinPosition;

pub struct DataTableState<T: Clone + PartialEq + Send + Sync + 'static> {
    pub current_page:    RwSignal<usize>,
    pub sort_column:     RwSignal<Option<String>>,
    pub sort_ascending:  RwSignal<bool>,
    pub filter_query:    RwSignal<String>,
    pub loading:         RwSignal<bool>,
    pub error:           RwSignal<Option<String>>,
    pub data:            RwSignal<Vec<T>>,
    pub total:           RwSignal<usize>,
    pub column_widths:   RwSignal<std::collections::HashMap<String, u32>>,
    pub pinned_columns:  RwSignal<std::collections::HashMap<String, PinPosition>>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> DataTableState<T> {
    pub fn new() -> Self {
        Self {
            current_page:   RwSignal::new(1usize),
            sort_column:    RwSignal::new(None::<String>),
            sort_ascending: RwSignal::new(true),
            filter_query:   RwSignal::new(String::new()),
            loading:        RwSignal::new(false),
            error:          RwSignal::new(None::<String>),
            data:           RwSignal::new(Vec::<T>::new()),
            total:          RwSignal::new(0usize),
            column_widths:  RwSignal::new(std::collections::HashMap::new()),
            pinned_columns: RwSignal::new(std::collections::HashMap::new()),
        }
    }
}
