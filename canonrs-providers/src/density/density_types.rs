use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DensityLevel {
    Compact,
    Normal,
    Comfortable,
}

#[derive(Clone, Copy)]
pub struct DensityContext {
    pub level: RwSignal<DensityLevel>,
}
