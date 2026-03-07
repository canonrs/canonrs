use leptos::prelude::*;
use canonrs_core::primitives::BadgePrimitive;
pub use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn Badge(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(default = false)] interactive: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <BadgePrimitive variant=variant interactive=interactive class=class id=id>
            {children.map(|c| c())}
        </BadgePrimitive>
    }
}
