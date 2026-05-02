//! slot! macro — ergonomic ChildrenFn helper
//!
//! Eliminates Arc::new boilerplate for block/layout slots.
//!
//! Usage:
//!   slot!(|| view! { <Foo /> }.into_any())
//!   slot!(move || view! { <Foo value=val /> }.into_any())

#[macro_export]
macro_rules! slot {
    ($closure:expr) => {
        std::sync::Arc::new($closure) as leptos::prelude::ChildrenFn
    };
}
