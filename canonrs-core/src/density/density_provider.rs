use leptos::prelude::*;
use super::density_types::{DensityContext, DensityLevel};

#[component]
pub fn DensityProvider(children: Children) -> impl IntoView {
    let level = create_rw_signal(DensityLevel::Normal);
    provide_context(DensityContext { level });
    
    children()
}
