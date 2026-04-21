use leptos::prelude::*;
use canonrs_core::infra::uid::generate;
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};

#[component]
pub fn StatGroupBlock(
    #[prop(optional)] stats: Option<ChildrenFn>,
    #[prop(default = GridCols::Three)] cols: GridCols,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid   = generate("bl");
    let stats = StoredValue::new(stats);
    view! {
        <section data-rs-stat-group="" data-rs-uid=uid aria-label="Statistics" class=class>
            <Grid cols=cols>
                {move || stats.get_value().map(|s| view! { <div data-rs-region="stats">{s()}</div> })}
            </Grid>
        </section>
    }
}
