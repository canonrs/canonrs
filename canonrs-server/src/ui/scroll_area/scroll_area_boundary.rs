//! ScrollArea Island — Canon Rule passthrough
use leptos::prelude::*;
use super::scroll_area_ui::ScrollArea as ScrollAreaUi;
pub use canonrs_core::primitives::ScrollOrientation;

#[component]
pub fn ScrollArea(
    children: Children,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
    #[prop(into, default = String::new())] viewport_id: String,
) -> impl IntoView {
    view! {
        <ScrollAreaUi style=style orientation=orientation auto_hide=auto_hide class=class viewport_id=viewport_id>
            {children()}
        </ScrollAreaUi>
    }
}
