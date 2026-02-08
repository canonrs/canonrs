use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
    None,
}

impl SortDirection {
    pub fn toggle(&self) -> Self {
        match self {
            Self::None => Self::Asc,
            Self::Asc => Self::Desc,
            Self::Desc => Self::None,
        }
    }
}

pub struct DataTableColumn<T>
where
    T: Clone + 'static,
{
    pub id: String,
    pub label: String,
    pub render: Box<dyn Fn(&T) -> AnyView + Send + Sync>,
    pub sortable: bool,
    pub width: Option<String>,
}

impl<T: Clone + 'static> DataTableColumn<T> {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            render: Box::new(|_| view! { "" }.into_any()),
            sortable: false,
            width: None,
        }
    }

    pub fn render<F, V>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> V + Send + Sync + 'static,
        V: IntoView + 'static,
    {
        self.render = Box::new(move |item| f(item).into_any());
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
}
