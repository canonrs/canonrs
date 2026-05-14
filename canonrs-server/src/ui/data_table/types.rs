//! DataTable semantic types — newtype wrappers para compatibilidade com Leptos macro

use std::sync::Arc;
use leptos::prelude::AnyView;

pub struct RowIdFn<T>(pub Option<Arc<dyn Fn(&T) -> String + Send + Sync>>);
pub struct RowLabelFn<T>(pub Option<Arc<dyn Fn(&T) -> String + Send + Sync>>);
pub struct ExpandRenderFn<T>(pub Option<Arc<dyn Fn(&T) -> AnyView + Send + Sync>>);

impl<T> Default for RowIdFn<T> {
    fn default() -> Self { Self(None) }
}
impl<T> Default for RowLabelFn<T> {
    fn default() -> Self { Self(None) }
}
impl<T> Default for ExpandRenderFn<T> {
    fn default() -> Self { Self(None) }
}
