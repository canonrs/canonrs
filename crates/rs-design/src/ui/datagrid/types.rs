use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Column definition
#[derive(Clone)]
pub struct ColumnDef<T> 
where
    T: Clone + Send + Sync + 'static,
{
    /// Column ID (unique)
    pub id: String,
    
    /// Column header label
    pub header: String,
    
    /// Accessor function to get value from row
    pub accessor: fn(&T) -> String,
    
    /// Optional width (CSS value)
    pub width: Option<String>,
    
    /// Is sortable?
    pub sortable: bool,
    
    /// Is filterable?
    pub filterable: bool,
    
    /// Custom cell renderer
    pub cell_renderer: Option<fn(&T) -> AnyView>,
}

impl<T> ColumnDef<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create new column
    pub fn new(id: impl Into<String>, header: impl Into<String>, accessor: fn(&T) -> String) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            accessor,
            width: None,
            sortable: false,
            filterable: false,
            cell_renderer: None,
        }
    }
    
    /// Set width
    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
    
    /// Make sortable
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
    
    /// Make filterable
    pub fn filterable(mut self) -> Self {
        self.filterable = true;
        self
    }
    
    /// Set custom cell renderer
    pub fn with_cell_renderer(mut self, renderer: fn(&T) -> AnyView) -> Self {
        self.cell_renderer = Some(renderer);
        self
    }
}

/// Sort direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

/// Sort state
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortState {
    pub column_id: String,
    pub direction: SortDirection,
}

/// Filter state
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterState {
    pub column_id: String,
    pub value: String,
}

/// Selection mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

/// DataGrid configuration
#[derive(Clone, Debug)]
pub struct DataGridConfig {
    /// Enable sorting
    pub sortable: bool,
    
    /// Enable filtering
    pub filterable: bool,
    
    /// Enable pagination
    pub pagination: bool,
    
    /// Rows per page
    pub page_size: usize,
    
    /// Selection mode
    pub selection_mode: SelectionMode,
    
    /// Enable row hover
    pub hoverable: bool,
    
    /// Enable striped rows
    pub striped: bool,
}

impl Default for DataGridConfig {
    fn default() -> Self {
        Self {
            sortable: true,
            filterable: false,
            pagination: false,
            page_size: 10,
            selection_mode: SelectionMode::None,
            hoverable: true,
            striped: false,
        }
    }
}

impl DataGridConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_sorting(mut self, enabled: bool) -> Self {
        self.sortable = enabled;
        self
    }
    
    pub fn with_filtering(mut self, enabled: bool) -> Self {
        self.filterable = enabled;
        self
    }
    
    pub fn with_pagination(mut self, page_size: usize) -> Self {
        self.pagination = true;
        self.page_size = page_size;
        self
    }
    
    pub fn with_selection(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }
    
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
}
