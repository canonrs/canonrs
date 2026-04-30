pub mod dispatcher;

// re-exporta do canonrs-runtime para compatibilidade
pub use canonrs_runtime::registry;
pub use canonrs_runtime::scanner;
pub use canonrs_runtime::observer;
pub use canonrs_runtime::init_element;
pub use canonrs_runtime::scan_children;
pub use canonrs_runtime::scan_and_init;
