use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTableRequest {
    pub page: usize,
    pub page_size: usize,
    pub sort_column: Option<String>,
    pub sort_ascending: bool,
    pub filter_query: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DataTableResponse<T: Clone + PartialEq> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Clone)]
pub struct ColumnDef<T> {
    pub id: String,
    pub label: String,
    pub sortable: bool,
    pub width: Option<String>,
    pub render: Arc<dyn Fn(&T) -> String + Send + Sync>,
}

// PartialEq manual - compara apenas id (suficiente para Memo)
impl<T> PartialEq for ColumnDef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.label == other.label
            && self.sortable == other.sortable
            && self.width == other.width
    }
}

impl<T> ColumnDef<T> {
    pub fn new(
        id: impl Into<String>, 
        label: impl Into<String>, 
        render: impl Fn(&T) -> String + Send + Sync + 'static
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            sortable: true,
            width: None,
            render: Arc::new(render),
        }
    }
    
    pub fn width(mut self, w: impl Into<String>) -> Self {
        self.width = Some(w.into());
        self
    }
    
    pub fn sortable(mut self, s: bool) -> Self {
        self.sortable = s;
        self
    }
}
