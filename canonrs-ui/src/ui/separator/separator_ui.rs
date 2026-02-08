use leptos::prelude::*;
use crate::primitives::SeparatorPrimitive;
use crate::shared::Orientation;

#[component]
pub fn Separator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = true)] decorative: bool,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("separator orientation-{} {}", orientation.as_str(), class);

    view! {
        <SeparatorPrimitive
            orientation={orientation.as_str().to_string()}
            decorative={decorative}
            aria_label={aria_label}
            class={base_class}
            id={id}
        />
    }
}
