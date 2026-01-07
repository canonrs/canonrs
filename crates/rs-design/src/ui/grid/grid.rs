use leptos::prelude::*;
use super::types::{GridCols, GridGap};
use super::variants::*;

#[component]
pub fn Grid(
    #[prop(default = GridCols::Responsive)] cols: GridCols,
    #[prop(default = GridGap::Md)] gap: GridGap,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let grid_classes = format!(
        "grid {} {}",
        cols.to_class(),
        gap.to_class()
    );

    view! {
        <div class=format!("{} {}", grid_classes, class)>
            {children()}
        </div>
    }
}
